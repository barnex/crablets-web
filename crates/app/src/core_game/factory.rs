use crate::prelude::*;
use core_game::{Error, Result};

#[derive(Serialize, Deserialize, Reflect, Clone)]
pub struct Factory {
	pub position: vec2i,
	pub scale: u8,
	pub sprite: Sprite,
	pub inputs: Vec<Input>,
	pub outputs: Vec<Output>,
	pub priority: u8,
	pub logic: FactoryLogic,
}

#[derive(Serialize, Deserialize, Reflect, Copy, Clone, Debug)]
pub enum FactoryLogic {
	Produce,
	BuildingSite { factory_prototype: usize },
	DigSite { tile: Tile },
}

impl Default for FactoryLogic {
	fn default() -> Self {
		Self::Produce
	}
}

impl Factory {
	pub fn can_reserve_input(&self, resource: Resource) -> bool {
		match self.input(resource) {
			None => false,
			Some(input) => !input.is_virtually_full(),
		}
	}

	pub fn reserve_input(&mut self, resource: Resource) -> Result<()> {
		match self.input_mut(resource) {
			None => Err(Error::Mismatch),
			Some(input) => input.reserve_slot(),
		}
	}

	pub fn add_input(&mut self, resource: Resource) -> Result<()> {
		match self.input_mut(resource) {
			None => Err(Error::Mismatch),
			Some(input) => input.add_to(),
		}
	}

	pub fn can_reserve_output(&self, resource: Resource) -> bool {
		match self.output(resource) {
			None => false,
			Some(input) => !input.is_vitrually_empty(),
		}
	}

	pub fn reserve_output(&mut self, resource: Resource) -> Result<()> {
		match self.output_mut(resource) {
			None => Err(Error::Mismatch),
			Some(input) => input.reserve_item(),
		}
	}

	pub fn take_output(&mut self, resource: Resource) -> Result<()> {
		match self.output_mut(resource) {
			None => Err(Error::Mismatch),
			Some(output) => output.take_item(),
		}
	}

	fn input_mut(&mut self, resource: Resource) -> Option<&mut Input> {
		self.inputs.iter_mut().find(|input| input.resource() == resource)
	}

	fn input(&self, resource: Resource) -> Option<&Input> {
		self.inputs.iter().find(|input| input.resource() == resource)
	}

	fn output_mut(&mut self, resource: Resource) -> Option<&mut Output> {
		self.outputs.iter_mut().find(|input| input.resource() == resource)
	}

	fn output(&self, resource: Resource) -> Option<&Output> {
		self.outputs.iter().find(|input| input.resource() == resource)
	}

	pub fn is_pile(&self) -> bool {
		self.inputs.len() == 1 && //_
		self.outputs.len() == 1 && self.inputs[0].resource() == self.outputs[0].resource()
	}

	pub fn bounds(&self) -> Bounds2i {
		Bounds {
			min: self.position,
			max: self.position + (self.scale as i32),
		}
	}

	pub fn tick(&mut self, gs: &mut GameState) {
		match self.logic {
			FactoryLogic::Produce => self.tick_produce(gs),
			FactoryLogic::BuildingSite { factory_prototype } => self.tick_building_site(gs, factory_prototype),
			FactoryLogic::DigSite { tile } => self.tick_dig_site(gs, tile),
		}
	}

	pub fn tick_dig_site(&mut self, gs: &mut GameState, tile: Tile) {
		if self.inputs.iter().all(Input::is_physically_full) {
			*gs.tilemap.at_mut(self.position.as_u32()) = tile;
			let position = self.position;
			// remove self via borrow checker dance
			gs.commands.push(Box::new(move |gs| {
				if let Some(i) = gs.factories.iter().position(|fac| fac.position == position) {
					gs.factories.swap_remove(i);
				}
			}));
		}
	}

	pub fn tick_building_site(&mut self, gs: &mut GameState, factory_prototype: usize) {
		if self.inputs.iter().all(Input::is_physically_full) {
			*self = FACTORY_PROTOTYPES[factory_prototype].1.clone().with(|fac| fac.position = self.position)
		}
	}

	pub fn tick_produce(&mut self, gs: &mut GameState) {
		// rudimentary: immediate 1:1 production
		// TODO: I/O ratio
		// TODO: take time
		if self.inputs.iter().all(|input| !input.is_empty()) && self.outputs.iter().all(|output| !output.is_full()) {
			self.inputs.iter_mut().for_each(|input| input.consume_items(1).expect("bug"));
			self.outputs.iter_mut().for_each(|output| output.produce_items(1).expect("bug"));
		}
	}
}

impl fmt::Debug for Factory {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Factory@{}[{}->{}]",
			self.position,
			self.inputs.iter().map(|buf| format!("{:?}", buf.resource())).join(","),
			self.outputs.iter().map(|buf| format!("{:?}", buf.resource())).join(","),
		)
	}
}

impl GameState {
	pub fn factory_at(&mut self, position: vec2i) -> Option<&mut Factory> {
		self.factories.iter_mut().find(|fac| fac.bounds().contains(position))
	}

	pub fn factory_id_at(&mut self, position: vec2i) -> Option<usize> {
		self.factories.iter().position(|fac| fac.bounds().contains(position))
	}
}

use crate::prelude::*;

#[derive(Serialize, Deserialize, Reflect, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Resource {
	index: u8,
}

#[allow(unused)]
impl Resource {
	pub const SEAWEED: Self = Self { index: 1 };
	pub const ROCK: Self = Self { index: 2 };
	pub const DRYWEED: Self = Self { index: 3 };
	pub const BRICK: Self = Self { index: 4 };
	pub const COAL: Self = Self { index: 5 };

	pub fn sprite(&self) -> Sprite {
		Sprite(16 + self.index as u16)
	}
}

impl fmt::Debug for Resource {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::SEAWEED => write!(f, "SEAWEED"),
			Self::ROCK => write!(f, "ROCK"),
			Self::DRYWEED => write!(f, "DRYWEED"),
			Self::BRICK => write!(f, "BRICK"),
			Self::COAL => write!(f, "COAL"),
			Resource { index } => write!(f, "Resource({index})"),
		}
	}
}

impl GameState {
	pub fn draw_resources(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈

		for pos in self.visible_tile_range().iter_excl() {
			let Some(resource) = self.resources.at(pos.as_u32()) else { continue };
			let sprite = resource.sprite();
			let position = pos.as_f32() + 0.5;
			sg.instances.push(InstanceData::new(position, sprite));
		}
	}

	//pub fn try_pick_up_resource(&mut self, resource: Resource, position: vec2i) -> Option<Resource> {
	//	if let Some(v) = self.resource_reservations.try_at_mut(position) {
	//		v.saturating_dec()
	//	}
	//	self.resources.try_at_mut(position)?.take()
	//}

	#[must_use]
	pub fn try_pick_up_any(&mut self, position: vec2i) -> Option<Resource> {
		if let Some(v) = self.resource_reservations.try_at_mut(position) {
			v.saturating_dec()
		}
		self.resources.try_at_mut(position)?.take()
	}
}

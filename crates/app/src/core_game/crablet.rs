use crate::prelude::*;

#[derive(Serialize, Deserialize, Reflect, Clone)]
pub struct Crablet {
	// tile position on the map
	pub position: vec2i,

	pub home_area: vec2i,

	// keeps previous tile congested until fully moved away
	pub prev_pos: vec2i,

	// animated position smoothly trails behind tile position
	pub anim_pos: vec2f,
	pub anim_timer: Timer,

	// movement state
	pub navigation: NavState,

	// work state
	pub task: Task,
	pub cargo: Option<Resource>,

	pub selected: bool,

	pub scale: f32,
	pub rotation: f32,
	pub sprite: Sprite,
}

impl Crablet {
	pub const SIZE: f32 = 0.8;

	pub fn new(position: vec2i) -> Self {
		Self {
			position,
			home_area: position,
			prev_pos: position,
			anim_pos: position.as_f32(),
			scale: Self::SIZE,
			rotation: 0.0,
			sprite: Sprite::FERRIS,
			selected: false,
			anim_timer: default(),
			navigation: default(),
			task: Task::None,
			cargo: None,
		}
	}

	pub fn tick(&mut self, gs: &mut GameState) {
		if !self.tick_animation(gs) {
			return;
		};

		match self.tick_navigation(gs) {
			NavStatus::Travelling => (), // keep ticking, perhaps next time will be done
			NavStatus::DestinationReached => self.tick_task(gs),
			NavStatus::Unreachable(dst) => self.on_destination_unreachable(gs, dst),
		}
	}

	//////////////////////////////// TASK

	fn on_destination_unreachable(&mut self, gs: &mut GameState, dst: vec2i) {
		gs.reserve_resource(dst); // Keep self/others from immediately coming back here
		self.task = Task::None;
		self.navigation = NavState::DestinationReached;
	}

	fn tick_task(&mut self, gs: &mut GameState) {
		match self.task {
			Task::Harvest { resource, from, to } => self.tick_harvest(gs, resource, from, to),
			Task::FactoryPickUp { resource, from, to } => self.tick_factory_pick_up(gs, resource, from, to),
			Task::FactoryDeliver { to } => self.tick_factory_deliver(gs, to),
			Task::None => self.tick_relax(gs),
		}
	}

	fn tick_harvest(&mut self, gs: &mut GameState, resource: Resource, from: vec2i, to: vec2i) {
		if self.position != from {
			return self.set_destination(from);
		}

		self.task = Task::None; // 👈 ensure task is cleared, unless replaced by Deliver.

		if let Some(ptr) = gs.resources.try_at_mut(self.position) {
			if *ptr == Some(resource) {
				*ptr = None;
				*gs.resource_reservations.at_mut(self.position.as_u32()) = 0;
				self.cargo = Some(resource);
				self.set_destination(to);
				self.task = Task::FactoryDeliver { to };
			}
		}
	}

	fn tick_factory_pick_up(&mut self, gs: &mut GameState, resource: Resource, from: vec2i, to: vec2i) {
		if self.position != from {
			return self.set_destination(from);
		}

		if let Some(fac) = gs.factory_at(self.position) {
			if fac.take_output(resource).is_ok() {
				self.cargo = Some(resource);
				self.task = Task::FactoryDeliver { to };
			} else {
				// Factory is empty, go do something else.
				self.task = Task::None;
			}
		} else {
			// Factory exploded? Go do something else.
			self.task = Task::None
		}
	}

	fn tick_factory_deliver(&mut self, gs: &mut GameState, to: vec2i) {
		if self.position != to {
			return self.set_destination(to);
		}

		let Some(cargo) = self.cargo else {
			return self.task = Task::None;
		};

		if let Some(fac) = gs.factory_at(self.position) {
			if fac.add_input(cargo).is_ok() {
				self.cargo = None;
			}
		}
		self.task = Task::None; // for succes OR failure (failure will get picked up by taksman::empty_hands)
	}

	pub fn tick_relax(&mut self, gs: &mut GameState) {
		if !gs.debug.enable_congestion {
			return;
		}

		gs.stats.inc(Event::CallRelax);
		let c = gs.congestion.at_or_default(self.position);
		if c > 1 {
			let mut rng = rand::thread_rng();
			if rng.gen_range(0..3) == 0 {
				if let Some(free_tile) = walkable_neighbours8(gs, self.position)
					.map(|pos| (pos, gs.congestion.at_or_default(pos)))
					.filter(|(pos, cong)| *cong <= c)
					.map(|(pos, cong)| (pos, cong + rng.gen_range(0..=2)))
					.min_by_key(|(pos, cong)| *cong)
					.map(|(pos, cong)| pos)
				{
					self.position = free_tile;
				}
			}
		}
	}

	//////////////////////////////// ANIMATE

	#[must_use = "returns true when current animation finished"]
	fn tick_animation(&mut self, gs: &mut GameState) -> bool {
		if !gs.debug.enable_animation {
			self.anim_pos = self.position.as_f32();
			self.prev_pos = self.position;
			return true;
		}

		gs.stats.inc(Event::CallAnimateCrab);

		let dst = self.position.center();
		let to_dst = dst - self.anim_pos;

		//wrong must step to linterp(time, total)

		let step_size = 1.0 / (gs.anim_frames_per_tile as f32); // << TODO mul by dt.
		if to_dst.len() < step_size {
			self.anim_pos = dst;
		} else {
			self.anim_pos += step_size * to_dst //.normalized();
		}

		if self.anim_timer.finished(gs.now) {
			self.anim_timer.set_alarm(gs.now, self.base_anim_time(gs));
			self.prev_pos = self.position;
			true
		} else {
			false
		}
	}

	fn base_anim_time(&self, gs: &mut GameState) -> u64 {
		(gs.anim_frames_per_tile + gs.rng().gen_range(0..=1)) as u64
	}

	//////////////////////////////// NAVIGATE

	pub fn set_destination(&mut self, destination: vec2i) {
		self.navigation.set_destination(destination)
	}

	#[must_use = "returns true if navigation done"]
	fn tick_navigation(&mut self, gs: &mut GameState) -> NavStatus {
		gs.stats.inc(Event::CallNavigateCrab);
		if let Some(next_tile) = self.navigation.tick(gs, self.position) {
			self.set_pos(gs, next_tile);
		}
		self.navigation.status()
	}

	fn set_pos(&mut self, gs: &mut GameState, position: vec2i) {
		// BAD: need to keep origin tile occupied until end of animation.
		if gs.debug.enable_congestion {
			let congestion = gs.congestion.at_or_default(position);
			self.anim_timer.set_alarm(gs.now, (congestion as u64 + 1) * self.base_anim_time(gs))
		}
		self.position = position;
	}
}

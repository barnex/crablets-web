use crate::prelude::*;
use core_game::{Error, Result};

#[derive(Clone, Reflect, Serialize, Deserialize)]
pub enum NavState {
	DestinationReached,
	Unreachable(vec2i),
	Travelling(Navigation),
}

pub enum NavStatus {
	DestinationReached,
	Unreachable(vec2i),
	Travelling,
}

impl NavStatus {
	pub fn is_travelling(&self) -> bool {
		matches!(self, Self::Travelling)
	}
	pub fn is_idle(&self) -> bool {
		!self.is_travelling()
	}
}

impl Default for NavState {
	fn default() -> Self {
		Self::DestinationReached
	}
}

impl NavState {
	pub fn set_destination(&mut self, destination: vec2i) {
		*self = Self::Travelling(Navigation::new(destination))
	}

	#[must_use = "returns next position"]
	pub fn tick(&mut self, gs: &mut GameState, position: vec2i) -> Option<vec2i> {
		let (next, dst) = match self {
			Self::DestinationReached => (Ok(None), None),
			Self::Unreachable(dst) => (Err(Error::Unreachable), Some(*dst)),
			Self::Travelling(nav) => (nav.tick(gs, position), Some(nav.destination)),
		};

		match next {
			Ok(None) => *self = Self::DestinationReached,
			Ok(Some(_)) => (),
			Err(_) => *self = Self::Unreachable(dst.expect("bug")),
		};

		next.ok().flatten()
	}

	pub fn status(&self) -> NavStatus {
		match self {
			NavState::DestinationReached => NavStatus::DestinationReached,
			NavState::Unreachable(dst) => NavStatus::Unreachable(*dst),
			NavState::Travelling(_) => NavStatus::Travelling,
		}
	}

	pub fn draw_on(&self, sg: &mut Scenegraph) {
		match self {
			Self::Travelling(nav) => nav.draw_on(sg),
			_ => (),
		}
	}
}

#[derive(Serialize, Deserialize, Reflect, Clone, Default)]
pub struct Navigation {
	destination: vec2i,
	pivot: Option<Pivot>,
	unreachable: bool,
}

#[derive(Serialize, Deserialize, Reflect, Clone)]
struct Pivot {
	pivot_pos: vec2i,
	best_dist: i32,
	start_pos: vec2i,
	//handedness: i8,
}

impl Navigation {
	fn new(destination: vec2i) -> Self {
		Self {
			destination,
			pivot: None,
			unreachable: false,
		}
	}

	fn tick(&mut self, gs: &mut GameState, position: vec2i) -> Result<Option<vec2i>> {
		if position == self.destination {
			return Ok(None);
		}

		let next = self
			.next_tile(gs, self.destination, position) //_
			.or_else(|| self.next_tile(gs, self.destination, position)) // 👈 retry if pivot changed
			.or_else(|| self.next_tile(gs, self.destination, position)) // 👈 retry if pivot changed
			.or_else(|| self.next_tile(gs, self.destination, position)) // 👈 retry if pivot changed
			.or_else(|| self.next_tile(gs, self.destination, position)); // 👈 retry if pivot changed

		//debug_assert!(next.is_some()); << fails if crab stuck on un-walkable island (e.g. drew beneath it).

		if self.pivot.as_ref().map(|p| p.start_pos) == next {
			gs.stats.inc(Event::NavigationStuck);
			return Err(Error::Unreachable);
		}

		Ok(next)
	}

	fn next_tile(&mut self, gs: &GameState, destination: vec2i, position: vec2i) -> Option<vec2i> {
		let next_tile_free = self.next_tile_free(gs, destination, position);
		match &mut self.pivot {
			// not currently pivoting obstacle: attempt to just step towards destination
			None => match next_tile_free {
				Some(next_tile) => Some(next_tile),
				None => {
					// path got blocked: attach to obstacle
					//  and start pivoting around it
					self.pivot = Pivot::start(gs, destination, position);
					None
				}
			},
			// currently pivoting obstacle
			Some(pivot) => {
				// pivoted strictly closer to destination: release pivot
				if let Some(next_tile) = next_tile_free {
					if next_tile.distance_squared(destination) < pivot.best_dist {
						self.pivot = None;
						return None;
					}
				}
				// keep pivoting around obstacle
				pivot.next_tile(gs, destination, position)
			}
		}
	}

	fn next_tile_free(&self, gs: &GameState, destination: vec2i, position: vec2i) -> Option<vec2i> {
		let curr_dist = position.distance_squared(destination);

		if gs.debug.enable_congestion {
			walkable_neighbours8(gs, position)
				.filter(|&pos| pos.distance_squared(destination) < curr_dist)
				.min_by_key(|&pos| (gs.congestion.at_or_default(pos), pos.distance_squared(destination)))
		} else {
			walkable_neighbours8(gs, position)
				.filter(|&pos| pos.distance_squared(destination) < curr_dist)
				.min_by_key(|&pos| pos.distance_squared(destination))
		}
	}

	pub fn draw_on(&self, sg: &mut Scenegraph) {
		if let Some(pivot) = &self.pivot {
			sg.instances.push(InstanceData::new(pivot.pivot_pos.as_f32() + 0.5, Sprite(0)).mix_color(vec4(1.0, 0.0, 0.0, 1.0)));
			sg.instances.push(InstanceData::new(pivot.start_pos.as_f32() + 0.5, Sprite(0)).mix_color(vec4(1.0, 0.0, 1.0, 1.0)));
		}
		sg.instances.push(InstanceData::new(self.destination.as_f32() + 0.5, Sprite(0)).mix_color(vec4(0.0, 1.0, 0.0, 1.0)));
	}
}

impl Pivot {
	#[must_use]
	fn start(gs: &GameState, destination: vec2i, position: vec2i) -> Option<Self> {
		let curr_dist = position.distance_squared(destination);
		let pivot_pos = neighbours(position)
			.into_iter()
			.filter(|&pos| !can_walk_tile(gs.tilemap.try_at(pos).unwrap_or_default()) && pos.distance_squared(destination) < curr_dist)
			.min_by_key(|pos| pos.distance_squared(destination));
		pivot_pos.map(|pivot_pos| Pivot {
			pivot_pos,
			start_pos: position,
			best_dist: position.distance_squared(destination),
			//handedness: match rand::thread_rng().gen() {
			//	true => 1,
			//	false => -1,
			//},
		})
	}

	/// TODO: Pivot keeps "ideal next tile"
	/// Actual next tile is in box around ideal next tile
	/// if freely reachable.
	/// So crabs won't bunch up against obstacles.
	#[must_use]
	fn next_tile(&mut self, gs: &GameState, destination: vec2i, position: vec2i) -> Option<vec2i> {
		let pivot_pos = self.pivot_pos;
		let pivot_vec = rot90(pivot_pos - position); // * self.handedness as i32;

		let mut position2 = position + pivot_vec;
		position2[0] = position2[0].clamp(pivot_pos[0] - 1, pivot_pos[0] + 1);
		position2[1] = position2[1].clamp(pivot_pos[1] - 1, pivot_pos[1] + 1);

		if gs.can_walk(position2) {
			Some(position2)
		} else {
			self.pivot_pos = position2;
			None
		}
	}
}

fn rot90(v: vec2i) -> vec2i {
	vec2i(v.y(), -v.x())
}

fn neighbours(pos: vec2i) -> [vec2i; 4] {
	[(0, -1), (-1, 0), (1, 0), (0, 1)].map(|v| pos + vec::from(v))
}

fn walkable_neighbours4(gs: &GameState, position: vec2i) -> impl Iterator<Item = vec2i> {
	let delta = [(0, -1), (-1, 0), (1, 0), (0, 1)].map(vec::from);
	let pos = delta.map(|v| position + v);
	let walk = pos.map(|v| gs.can_walk(v));
	let walkable = iter::zip(pos, walk).filter_map(|(pos, walk)| walk.then_some(pos)).collect::<SmallVec<[vec2i; 8]>>();

	walkable.into_iter()
}

pub fn walkable_neighbours8(gs: &GameState, position: vec2i) -> impl Iterator<Item = vec2i> {
	let delta = [(0, -1), (-1, 0), (1, 0), (0, 1)].map(vec::from);
	let pos = delta.map(|v| position + v);
	let walk = pos.map(|v| gs.can_walk(v));
	let mut walkable = iter::zip(pos, walk).filter_map(|(pos, walk)| walk.then_some(pos)).collect::<SmallVec<[vec2i; 8]>>();

	// can move diagonally if two direct neighbors are walkable
	for ((d1, w1), (d2, w2)) in iter::zip(delta, walk).circular_tuple_windows() {
		if w1 && w2 {
			let pos = position + d1 + d2;
			if gs.can_walk(pos) {
				walkable.push(pos)
			}
		}
	}

	walkable.into_iter()
}

impl GameState {
	pub fn can_walk(&self, tile_pos: vec2i) -> bool {
		self.tilemap.try_at(tile_pos).map(can_walk_tile).unwrap_or(false)
	}
}

pub fn can_walk_tile(tile: Tile) -> bool {
	!matches!(tile, Tile::WATER | Tile::STONE)
}

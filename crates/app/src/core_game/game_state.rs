use crate::prelude::*;

#[derive(Serialize, Deserialize, Reflect)]
#[serde(default)]
pub struct GameState {
	pub now: u64,

	pub camera: Camera,

	pub search_radius: i32,
	pub anim_frames_per_tile: u8, // <<< remove

	#[serde(skip)]
	pub mouse_pos: vec2f,
	pub dragging: Option<(vec2i, vec2i)>,

	pub tilemap: Tilemap,
	pub resources: Vec2D<Option<Resource>>,
	pub resource_reservations: Vec2D<u8>,
	pub congestion: Vec2D<u8>,
	pub crablets: Vec<Crablet>,
	pub factories: Vec<Factory>,
	pub grow_crops_rate: f32,

	pub debug: DebugOpts,

	#[serde(skip)]
	pub inputs: Inputs,

	#[reflect(ignore)]
	pub stats: Stats,

	pub ed_mode: EdMode,
	pub editor_current_tile: u8,
	pub editor_current_building: u8,

	#[reflect(ignore)]
	#[serde(skip)]
	pub rng: Option<Rng>,

	#[reflect(ignore)]
	#[serde(skip)]
	pub commands: Vec<Box<dyn FnOnce(&mut GameState) + Send + Sync>>,
}

pub const MAJOR_TICK: u64 = 16; // TODO

impl GameState {
	pub fn new() -> Self {
		default()
	}

	pub fn tick(&mut self) {
		if self.debug.pause_all_systems {
			return;
		}
		self.now += 1;
		self.stats.next_frame();

		profiler::scope("tick_inputs", || self.tick_inputs());
		profiler::scope("tick_congestion", || self.tick_congestion());
		profiler::scope("tick_resource_reservation", || self.tick_resource_reservation());
		profiler::scope("tick_crabs", || self.tick_crabs());
		profiler::scope("tick_factories", || self.tick_factories());
		profiler::scope("tick_taskman", || systems::tick_taskman(self));
		profiler::scope("tick_grow_crops", || systems::tick_grow_crops(self));

		self.exec_command_queue();
	}

	fn exec_command_queue(&mut self) {
		let commands = mem::take(&mut self.commands);
		for cmd in commands {
			cmd(self)
		}
	}

	pub fn tick_crabs(&mut self) {
		for i in 0..self.crablets.len() {
			// 👆 iterate without holding on to game state
			let mut mutable_crab = self.crablets[i].clone();
			mutable_crab.tick(self);
			self.crablets[i] = mutable_crab;
		}
	}

	pub fn rng(&mut self) -> &mut Rng {
		self.rng.as_mut().expect("rng")
	}

	pub fn visible_tile_range(&self) -> Bounds2i {
		self //.
			.camera
			.visible_tile_range()
			.map(|b| b.floor())
			.intersect(self.tilemap.bounds().as_i32())
	}
}

impl Default for GameState {
	fn default() -> Self {
		let map_size = vec2u(256, 128);
		let rng = Rng::seed_from_u64(123);
		Self {
			now: 0,
			stats: default(),
			mouse_pos: default(),
			inputs: default(),
			dragging: None,
			search_radius: 48,
			anim_frames_per_tile: 24,
			tilemap: Tilemap::new(map_size, Tile::FARM_LAND),
			resources: Vec2D::new(map_size, None),
			resource_reservations: Vec2D::new(map_size, 0),
			congestion: Vec2D::new(map_size, 0),
			//buildings: vec![BUILDING_PROTOTYPES[0].clone().1.with(|b| b.position = vec2u(4, 6))],
			factories: default(),
			crablets: Vec::from_iter([
				Crablet::new(vec2(3, 5)),
				//Crablet::new(vec2(10, 5)).with(|c| c.work_state = WorkState::Carrying(Resource::ROCK)),
				//Crablet::new(vec2(5, 10)),
			]),
			camera: default(),
			rng: Some(rng),
			debug: default(),
			ed_mode: default(),
			editor_current_tile: default(),
			editor_current_building: default(),
			commands: default(),
			grow_crops_rate: 0.02,
		}
	}
}

pub fn index_atlas(sprite: Sprite) -> (vec2f, vec2f) {
	const LOG_SPRITES_PER_ROW: u16 = 3;
	const SPRITES_PER_ROW: u16 = 1 << LOG_SPRITES_PER_ROW;
	const STRIDE: f32 = SPRITES_PER_ROW as f32;

	let idx = sprite.0;
	let (ix, iy) = (idx & (SPRITES_PER_ROW - 1), idx >> LOG_SPRITES_PER_ROW);
	let (ix, iy) = (ix as f32, iy as f32);

	let tex_coords_off = vec2f(ix / STRIDE, 1.0 - (iy + 1.0) / STRIDE);
	let tex_coords_size = vec2::splat(1.0 / STRIDE);
	(tex_coords_off, tex_coords_size)
}

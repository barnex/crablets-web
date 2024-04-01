use crate::prelude::*;

impl App {
	pub fn command_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("$> cmd") //.
			.open(&mut self.ui_state.commands_open)
			.show(ctx, |ui| {
				Self::cmd_button(ui, "grow_crops", || systems::instantly_grow_crops(&mut self.gamestate));
				Self::cmd_button(ui, "spawn_rocks", || self.gamestate.spawn_rocks());
				Self::cmd_button(ui, "give_rocks", || self.gamestate.give_rocks());

				Self::cmd_button(ui, "spawn", || self.gamestate.spawn());
				Self::cmd_button(ui, "spawn_grid", || self.gamestate.spawn_grid(10));
				Self::cmd_button(ui, "remove", || self.gamestate.remove_selected());
				Self::cmd_button(ui, "decimate", || self.gamestate.decimate_crabs());

				Self::cmd_button(ui, "tick_factories", || self.gamestate.tick_factories());
				Self::cmd_button(ui, "drain factories", || self.gamestate.drain_factories());

				Self::cmd_button(ui, "tick_crabs", || self.gamestate.tick_crabs());
				Self::cmd_button(ui, "tick_empty_hands", || systems::tick_empty_hands(&mut self.gamestate));
				//Self::cmd_button(ui, "tick_pick_up", || systems::tick_pick_up(&mut self.gamestate));
				//Self::cmd_button(ui, "tick_drop_off", || systems::tick_drop_off(&mut self.gamestate));
				Self::cmd_button(ui, "tick_give_work", || systems::tick_give_work(&mut self.gamestate));
			});
	}

	fn cmd_button(ui: &mut Ui, text: &str, f: impl FnMut()) {
		ui.button(text).clicked().then(f);
	}
}

impl GameState {
	pub fn remove_selected(&mut self) {
		let mut i = 0;
		while i < self.crablets.len() {
			if self.crablets[i].selected {
				self.crablets.swap_remove(i);
			} else {
				i += 1
			}
		}
	}

	pub fn spawn(&mut self) {
		let rng = self.rng();
		let pos = (20.0 * vec2f(rng.gen(), rng.gen()) + self.camera.world_position).floor();
		self.crablets.push(Crablet::new(pos));
	}

	pub fn spawn_grid(&mut self, n: u32) {
		let n = n as i32;
		for position in (0..n).cartesian_product(0..n).map(vec2::from) {
			self.crablets.push(Crablet::new(position));
		}
	}

	pub fn decimate_crabs(&mut self) {
		self.crablets.truncate(self.crablets.len() / 2);
	}

	pub fn drain_factories(&mut self) {
		for buf in self.factories.iter_mut().flat_map(|build| build.inputs.iter_mut()) {
			buf.drain();
			// reserved slots stay in place
			// because crabs who are on the way to deliver/pick-up are still on the way
		}
		for buf in self.factories.iter_mut().flat_map(|build| build.outputs.iter_mut()) {
			buf.drain();
			// reserved slots stay in place
			// because crabs who are on the way to deliver/pick-up are still on the way
		}
	}

	pub fn give_rocks(&mut self) {
		for crab in &mut self.crablets {
			crab.cargo = Some(Resource::ROCK)
		}
	}

	pub fn spawn_rocks(&mut self) {
		let mut rng = rand::thread_rng();
		for pos in self.resources.iter_positions() {
			(rng.gen::<f32>() < 0.01).then(|| self.resources.try_set(pos.as_i32(), Some(Resource::ROCK)));
		}
	}
}

use crate::prelude::*;

mod edit_window;
use edit_window::*;
mod inspector;
use inspector::*;
mod commands;

#[derive(Serialize, Deserialize)]
pub struct App {
	ui_state: UiState,

	gamestate: GameState,
	update_scenegraph: bool,
	#[serde(skip)]
	scenegraph: Scenegraph,

	canvas: EguiCanvas,
}

#[derive(Serialize, Deserialize, Default, Reflect)]
struct UiState {
	gamestate_open: bool,
	scenegraph_open: bool,
	ui_state_open: bool,
	scope_open: bool,
	profiler_open: bool,
	stats_open: bool,
	commands_open: bool,
	edit_open: bool,
	dark_mode: bool,

}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		let settings = Settings::todo_load_settings();
		Self::try_restore(cc, &settings).unwrap_or_else(|| Self::default(cc, &settings))
	}

	fn default(cc: &eframe::CreationContext<'_>, settings: &Settings) -> Self {
		Self {
			canvas: EguiCanvas::new(cc, &settings.graphics),
			gamestate: GameState::default(),
			ui_state: UiState::default(),
			scenegraph: default(),
			update_scenegraph: true,
		}
	}

	// Load previous app state (if any).
	fn try_restore(cc: &eframe::CreationContext<'_>, settings: &Settings) -> Option<Self> {
		let storage = cc.storage?;
		let restored = eframe::get_value::<Self>(storage, eframe::APP_KEY)?;
		restored.ui_state.dark_mode.then(|| cc.egui_ctx.set_visuals(egui::Visuals::dark()));
		log::info!("successfully restored from egui storage");

		Some(Self {
			canvas: EguiCanvas::new(cc, &settings.graphics), // 👈 hack to initialize wgpu resources
			..restored
		})
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		profiler::restart("frame time");

		self.gamestate.tick();

		profiler::scope("egui", || {
			self.top_panel(ctx);
			self.central_panel(ctx);
		});

		ctx.request_repaint();
	}

	fn top_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				ui.button("❌").on_hover_text("Quit").clicked().then(|| ctx.send_viewport_cmd(egui::ViewportCommand::Close));
				self.dark_mode_switch(ui);
				Self::toggle_button(ui, &mut self.ui_state.edit_open, "✏", "Show Editor");
				Self::toggle_button(ui, &mut self.ui_state.gamestate_open, "🔎", "Show gamestate");
				Self::toggle_button(ui, &mut self.ui_state.scenegraph_open, "🎬", "Show scenegraph");
				Self::toggle_button(ui, &mut self.ui_state.ui_state_open, "↗", "Show UI state");
				Self::toggle_button(ui, &mut self.ui_state.commands_open, "$>", "Show commands");
				Self::toggle_button(ui, &mut self.ui_state.scope_open, "~", "Show scope");
				Self::toggle_button(ui, &mut self.ui_state.profiler_open, "⏳", "Show profiler");
				Self::toggle_button(ui, &mut self.ui_state.stats_open, "📈", "Show stats");
			});
		});
	}

	fn central_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().frame(egui::Frame::default()).show(ctx, |ui| {
			self.gamestate_window(ctx);
			self.scenegraph_window(ctx);
			self.scope_window(ctx);
			self.profiler_window(ctx);
			self.stats_window(ctx);
			self.ui_state_window(ctx);
			self.command_window(ctx);
			self.edit_window(ctx);
			self.canvas(ctx, ui);
		});
	}

	fn canvas(&mut self, ctx: &egui::Context, ui: &mut Ui) {
		let rect = egui::Frame {
			fill: egui::Color32::from_gray(128),
			..default()
		}
		.show(ui, |ui| {
			let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::focusable_noninteractive());
			self.gamestate.camera.viewport_size_pix = vec2(rect.width(), rect.height()).as_u32();
			if self.update_scenegraph {
				profiler::scope("update_scenegraph", || {
					self.scenegraph.clear();
					self.gamestate.draw_on(&mut self.scenegraph);
				});
			}
			self.canvas.paint(ui, rect, self.scenegraph.clone());
			rect
		})
		.inner;

		if !ctx.is_using_pointer() {
			ctx.input(|inputs| self.gamestate.inputs.record(rect.left_top().into(), inputs));
		}
	}



	fn ui_state_window(&mut self, ctx: &egui::Context) {
		// borrow checker dance since ui_state window controls it's open openness.
		let mut inspector_open = self.ui_state.ui_state_open;
		egui::Window::new("↗ ui state") //.
			.open(&mut inspector_open)
			.show(ctx, |ui| {
				inspect(ui, "ui state", &mut self.ui_state);
			});
		self.ui_state.ui_state_open = inspector_open;
	}

	fn profiler_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("⏳ profiler") //.
			.open(&mut self.ui_state.profiler_open)
			.show(ctx, |ui| ui.add(egui::Label::new(profiler::to_string())));
	}

	fn stats_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("📈 stats") //.
			.open(&mut self.ui_state.stats_open)
			.show(ctx, |ui| {
				egui::Grid::new("some_unique_id").show(ui, |ui| {
					ui.label("stat");
					ui.label("frame");
					ui.label("total");
					ui.end_row();
					for (stat, frame, total) in self.gamestate.stats.iter() {
						ui.label(format!("{stat:?}"));
						ui.label(format!("{frame}"));
						ui.label(format!("{total}"));
						ui.end_row();
					}
					ui.end_row();
				})
			});
	}

	fn scope_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("~ scope") //.
			.open(&mut self.ui_state.scope_open)
			.show(ctx, |ui| ui.add(egui::Label::new(scope::to_string())));
	}

	fn scenegraph_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("🎬 scenegraph") //.
			.open(&mut self.ui_state.scenegraph_open)
			.show(ctx, |ui| {
				ui.checkbox(&mut self.update_scenegraph, "update");
				inspect(ui, "scenegraph", &mut self.scenegraph);
			});
	}

	fn gamestate_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("🔎 state") //.
			.open(&mut self.ui_state.gamestate_open)
			.show(ctx, |ui| {
				if ui.button("❌ reset").clicked() {
					self.gamestate = default();
				}
				inspect(ui, "state", &mut self.gamestate);
			});
	}

	fn toggle_button(ui: &mut Ui, state: &mut bool, label: &str, tooltip: &str) {
		ui.add(egui::SelectableLabel::new(*state, label)).on_hover_text(tooltip).clicked().then(|| toggle(state));
	}

	// Like egui's `global_light_dark_mode_switch`, but persists state.
	fn dark_mode_switch(&mut self, ui: &mut Ui) {
		let style: egui::Style = (*ui.ctx().style()).clone();
		let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
		if let Some(visuals) = new_visuals {
			self.ui_state.dark_mode = visuals.dark_mode; // 👈 persist
			ui.ctx().set_visuals(visuals);
		}
	}
}

impl eframe::App for App {
	// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		log::info!("persisting egui state");
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn on_exit(&mut self) {
		log::info!("exiting");
	}

	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		self.update(ctx, frame);
	}
}

fn toggle(ptr: &mut bool) {
	*ptr = !*ptr
}

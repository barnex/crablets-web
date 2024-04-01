use crate::prelude::*;

impl App {
	pub fn edit_window(&mut self, ctx: &egui::Context) {
		let tool = self.gamestate.ed_mode;
		let tile = self.gamestate.editor_current_tile;
		egui::Window::new("✏ Editor") //.
			.open(&mut self.ui_state.edit_open)
			.show(ctx, |ui| {
				// Tool buttons
				ui.horizontal(|ui| {
					let mut mode_button = |mode, icon, label| {
						ui.add(egui::SelectableLabel::new(tool == mode, icon))
							.on_hover_text(label)
							.clicked()
							.then(|| self.gamestate.ed_mode = mode);
					};
					mode_button(EdMode::Select, "↗", "manipulate crablets");
					mode_button(EdMode::Pencil, "✏", "draw tiles");
					mode_button(EdMode::Fill, "▭", "fill tiles");
					mode_button(EdMode::Buildings, "🏭", "buildings");
				});

				ui.separator();

				let mut tile_buttons = |ui: &mut Ui| {
					for row in 0..16 {
						ui.horizontal(|ui| {
							for col in 0..4 {
								let i = col + row * 4;
								ui.add(egui::SelectableLabel::new(i == tile, i.to_string())).clicked().then(|| self.gamestate.editor_current_tile = i);
							}
						});
					}
				};

				let mut building_buttons = |ui: &mut Ui| {
					for (i, (label, _)) in FACTORY_PROTOTYPES.iter().enumerate() {
						let i = i as u8;
						ui.add(egui::SelectableLabel::new(i == self.gamestate.editor_current_building, *label))
							.clicked()
							.then(|| self.gamestate.editor_current_building = i);
					}
				};

				ui.checkbox(&mut self.gamestate.debug.god_mode, "god mode");

				match self.gamestate.ed_mode {
					EdMode::Select => (),
					EdMode::Pencil => tile_buttons(ui),
					EdMode::Buildings => building_buttons(ui),
					EdMode::Fill => tile_buttons(ui),
				}
			});
	}
}

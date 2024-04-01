use crate::*;

impl GameState {
	pub fn tick_congestion(&mut self) {
		if self.debug.enable_congestion {
			self.congestion.clear();
			for i in 0..self.crablets.len() {
				let pos = self.crablets[i].position;
				let prev_pos = self.crablets[i].prev_pos;

				self.add_congestion(pos);

				if prev_pos != pos {
					self.add_congestion(prev_pos);
				}
			}
		}
	}

	pub fn add_congestion(&mut self, pos: vec2i) {
		let Some(v) = self.congestion.try_at_mut(pos) else { return };
		*v = v.saturating_add(1);
	}

	pub fn draw_congestion(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈

		let sprite = Sprite::FACTORY; // <<< Anything fully opque

		for index in self.visible_tile_range().iter_excl() {
			let value = self.congestion.at_or_default(index) as f32;
			let color = (((value / 255.0).sqrt()) * 2.0).clamp(0.0, 1.0);
			let position = index.as_f32() + 0.5;
			sg.instances.push(InstanceData::new(position, sprite).mix_color(vec4(color, color, color, 1.0)));
		}
	}
}

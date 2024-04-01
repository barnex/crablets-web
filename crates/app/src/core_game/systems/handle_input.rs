use crate::*;

impl GameState {
	pub fn tick_inputs(&mut self) {
		self.mouse_pos = self.camera.screen_to_tile(self.inputs.pointer_pos_pix);
		self.handle_zoom();
		self.handle_scroll();
		self.handle_mouse();
		self.inputs.reset();
	}

	fn handle_mouse(&mut self) {
		let pos = self.mouse_pos.floor();

		if self.inputs.mouse_just_pressed {
			self.dragging = Some((pos, pos))
		}

		if self.inputs.mouse_is_down {
			if let Some((_start, end)) = &mut self.dragging {
				*end = pos;
			}
			self.handle_mouse_drag(pos);
		}

		if self.inputs.mouse_just_released {
			match self.dragging.take() {
				None => (),
				Some(dragging) => self.handle_mouse_up(dragging),
			}
		}
	}

	fn handle_mouse_drag(&mut self, pos: vec2i) {
		match self.ed_mode {
			EdMode::Select => (),
			EdMode::Pencil => self.map_draw(pos),
			EdMode::Fill => (),
			EdMode::Buildings => (),
		}
	}

	fn handle_mouse_up(&mut self, (start, end): (vec2i, vec2i)) {
		match self.ed_mode {
			EdMode::Select => self.select_mode_mouse_up((start, end)),
			EdMode::Pencil => self.map_draw(end),
			EdMode::Fill => self.map_fill(Self::selection((start, end))),
			EdMode::Buildings => self.draw_factory(end, self.editor_current_building),
		}
	}

	fn select_mode_mouse_up(&mut self, (start, end): (vec2i, vec2i)) {
		let selection = Self::selection((start, end));
		if start != end {
			// dragged a rectangle: select crabs
			self.select_crabs(selection)
		} else {
			// just a click.
			// some crabs selected: send them to click location
			if self.crablets.iter().any(|crab| crab.selected) {
				for crab in self.crablets.iter_mut().filter(|crab| crab.selected) {
					crab.navigation.set_destination(start);
					crab.home_area = start;
					crab.selected = false;
				}
			} else {
				// no crabs selected: click crab we clicked on, if any
				self.select_crabs(selection);
			}
		}
	}

	fn select_crabs(&mut self, selection: Bounds2i) {
		for crab in &mut self.crablets {
			crab.selected = selection.contains(crab.position);
		}
	}

	fn map_draw(&mut self, pos: vec2i) {
		let tile = Tile(self.editor_current_tile);
		match self.debug.god_mode {
			true => self.tilemap.try_set(pos, tile),
			false => self.try_build_factory(dig_site_prototype(tile), pos),
		}
	}

	fn map_fill(&mut self, range: Bounds2i) {
		for pos in range.iter_excl() {
			self.map_draw(pos)
		}
	}

	fn draw_factory(&mut self, pos: vec2i, i: u8) {
		if !self.tilemap.bounds().as_i32().contains(pos) {
			return;
		}
		let prototype = match self.debug.god_mode {
			false => building_site_prototype(i as usize),
			true => FACTORY_PROTOTYPES[i as usize].1.clone(),
		};
		self.try_build_factory(prototype, pos);
	}

	fn try_build_factory(&mut self, prototype: Factory, position: vec2i) {
		let prototype = Factory { position, ..prototype };
		if self.factories.iter().any(|fac| fac.bounds().intersects(prototype.bounds())) {
			return; // no building on top of other buildings, please
		}
		self.factories.push(prototype);
	}

	fn handle_zoom(&mut self) {
		self.camera.zoom = (self.camera.zoom * self.inputs.zoom_delta).clamp(4.0, 256.0);
	}

	fn handle_scroll(&mut self) {
		// zoom-dependent sensitivity so that we pan by same amount of screen pixels
		// (fast zoom when zoomed out, slow when zoomed in)
		let sensitivity: f32 = 0.03 / self.camera.zoom;
		self.camera.world_position += self.inputs.scroll_delta * vec2(1.0, -1.0) * sensitivity;
		//self.gamestate.camera.position = self.gamestate.camera.position.map(|v| (v * 32.0).floor() / 32.0);
		// 👈 integer camera position
	}

	// Turn drag start, end into a selection rectangle.
	pub fn selection((start, end): (vec2i, vec2i)) -> Bounds2i {
		Bounds::from_point(start).add(end).make_inclusive()
	}
}

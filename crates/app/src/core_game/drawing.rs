use crate::*;

impl GameState {
	pub fn draw_on(&self, sg: &mut Scenegraph) {
		scope::record("camera.visible_tile_range", self.camera.visible_tile_range());

		sg.uniforms.camera = self.camera.matrix();

		self.draw_tilemap(sg);
		self.draw_factories(sg);
		if self.debug.congestion_overlay {
			self.draw_congestion(sg);
		}
		self.draw_resources(sg);
		self.draw_selection_rectangle(sg);
		self.draw_building_footprint(sg);
		self.draw_pen_cursor(sg);
		if self.debug.reservation_overlay {
			self.draw_resource_reservation(sg);
		}
		self.draw_crablets(sg);
	}

	pub fn draw_tilemap(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈

		for index in self.visible_tile_range().iter_excl() {
			let tile = self.tilemap.at(index.as_u32());
			let sprite = self.tilemap.sprites[tile.0 as usize];
			let position = index.as_f32() + 0.5;
			sg.instances.push(InstanceData::new(position, sprite));
		}
	}

	pub fn draw_factories(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈  draw atop tiles

		let visible_range = self //.
			.camera
			.visible_tile_range()
			.map(|b| b.floor());

		for Factory { position, scale, sprite, .. } in self.factories.iter().filter(|building| visible_range.contains(building.position.as_i32())) {
			let position = position.as_f32() + (*scale as f32 * 0.5);
			sg.instances.push(
				InstanceData::new(position, *sprite) //.
					.with(|d| d.scale = *scale as f32),
			)
		}

		sg.new_layer(); // 👈  draw atop tiles

		for Factory { position, scale, inputs, outputs, .. } in self.factories.iter().filter(|building| visible_range.contains(building.position.as_i32())) {
			let position = position.as_f32() + vec2(0.5, 0.5);
			let scale = *scale as f32;

			for (i, buf) in inputs.iter().enumerate() {
				buf.draw_on(sg, position + vec2f(i as f32, 0.0), scale);
			}
			for (i, buf) in outputs.iter().enumerate() {
				buf.draw_on(sg, position + vec2f((i + inputs.len()) as f32, 0.0), scale);
			}
		}
	}

	pub fn draw_crablets(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈  draw atop tiles

		const MAX_SPRITE_SIZE: f32 = 4.0; // tiles

		let visible_range = self //.
			.camera
			.visible_tile_range()
			.add_margin(MAX_SPRITE_SIZE);

		for crab in self.crablets.iter().filter(|crab| visible_range.contains(crab.anim_pos)) {
			const SELECT_COLOR: vec4f = vec4f(0.5, 0.5, 1.0, 0.6);
			sg.instances.push(InstanceData::new(crab.anim_pos, crab.sprite).with(|d| {
				d.mix_color = select(SELECT_COLOR, vec4f::ZERO, crab.selected);
				d.scale = crab.scale;
				d.rotation = crab.rotation;
			}));

			if self.debug.navigation_overlay {
				// draw tile position
				sg.instances.push(InstanceData::new(crab.position.center(), Sprite::RECTICLE).mix_color(vec4(1.0, 0.0, 0.0, 0.5)));
				crab.navigation.draw_on(sg);
			}

			if crab.selected {
				sg.instances.push(InstanceData::new(crab.home_area.center(), Sprite::RECTICLE).mix_color(vec4(1.0, 0.0, 0.0, 0.5)));
			}
		}

		sg.new_layer(); // 👈  draw atop tiles

		// draw resources (own layer so on top of crabs. TODO: use z order instead)
		const CARRIED_RESOURCE_SIZE: f32 = 0.75;
		for crab in self.crablets.iter().filter(|crab| visible_range.contains(crab.anim_pos)) {
			let Some(resource) = crab.cargo else { continue };
			let sprite = resource.sprite();
			sg.instances
				.push(InstanceData::new(crab.anim_pos - vec2(0.0, crab.scale / 4.0), sprite).with(|v| v.scale = CARRIED_RESOURCE_SIZE));
		}
	}

	pub fn draw_selection_rectangle(&self, sg: &mut Scenegraph) {
		match self.ed_mode {
			EdMode::Select => (),
			EdMode::Pencil => return,
			EdMode::Buildings => return,
			EdMode::Fill => (),
		}

		let Some(dragging) = self.dragging else { return };

		sg.new_layer();
		for pos in Self::selection(dragging).iter_excl() {
			sg.instances.push(InstanceData::new(pos.center(), Sprite(7)));
		}
	}

	pub fn draw_building_footprint(&self, sg: &mut Scenegraph) -> Option<()> {
		if !matches!(self.ed_mode, EdMode::Buildings) {
			return None;
		}

		sg.new_layer();
		let building = FACTORY_PROTOTYPES.get(self.editor_current_building as usize)?.1.clone().with(|b| b.position = self.mouse_pos.floor());

		let position = building.position.as_f32() + (building.scale as f32) * vec2(0.5, 0.5);
		sg.instances.push(
			InstanceData::new(position, building.sprite) //.
				.with(|d| d.scale = building.scale as f32)
				.mix_color(vec4(0.0, 1.0, 1.0, 0.5)),
		);
		Some(())
	}

	pub fn draw_pen_cursor(&self, sg: &mut Scenegraph) {
		match self.ed_mode {
			EdMode::Select => return,
			EdMode::Pencil => (),
			EdMode::Buildings => return,
			EdMode::Fill => (),
		}
		sg.new_layer();

		sg.instances.push(
			InstanceData::new(
				self.mouse_pos.floor().center(),
				self.tilemap.sprites.get(self.editor_current_tile as usize).copied().unwrap_or(Sprite(0)),
			) //.
			.mix_color(vec4(0.0, 1.0, 1.0, 0.5)),
		);
	}
}

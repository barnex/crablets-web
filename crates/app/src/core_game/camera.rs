use crate::prelude::*;

#[derive(Serialize, Deserialize, Reflect)]
pub struct Camera {
	/// screen size in pixels
	pub viewport_size_pix: vec2u,

	/// camera is centered on this game (tile) position
	pub world_position: vec2f,

	/// zoom factor between world (tiles) and screen (pixels)
	pub zoom: f32,
}

impl Camera {
	/// Transform matrix (world to screen)
	pub fn matrix(&self) -> mat4x4f {
		let (dx, dy) = self.world_position.into();
		let viewport_size = self.viewport_size_pix.as_f32();
		let (sx, sy) = (vec::splat(2.0 * self.zoom) / viewport_size).into();

		mat([
			[sx, 0.0, 0.0, 0.0],            //
			[0.0, sy, 0.0, 0.0],            //
			[0.0, 0.0, 1.0, 0.0],           //
			[-sx * dx, -sy * dy, 0.0, 1.0], //
		])
	}

	/// convert from screen position (pixels) to game (tile) position
	pub fn screen_to_tile(&self, pos: vec2f) -> vec2f {
		let tile_zoom = self.zoom;
		let x = (pos.x() - self.viewport_size_pix.x() as f32 / 2.0) / tile_zoom + self.world_position.x();
		let y = (-pos.y() + self.viewport_size_pix.y() as f32 / 2.0) / tile_zoom + self.world_position.y();
		vec2f(x, y)
	}

	/// visible part of the world, given current camera position, zoom and screen size.
	pub fn visible_tile_range(&self) -> Bounds2f {
		let vec([w, h]) = self.viewport_size_pix.as_f32();
		Bounds2f {
			min: self.screen_to_tile(vec2(0.0, h)),
			max: self.screen_to_tile(vec2(w, 0.0)) + vec::ONES,
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			world_position: default(),
			zoom: 64.0,
			viewport_size_pix: vec::ONES,
		}
	}
}

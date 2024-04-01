use crate::prelude::*;

/// Data passed to each instance for instanced rendering.
/// Used by shaders as `instance_data`.
///
/// ! `repr(C)` required by WGPU.
/// ! Must be kept in sync with shaders.
/// ! Fields must be aligned to WGPU requirements
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable, Reflect)]
pub struct InstanceData {
	pub mix_color: vec4f,       // 4
	pub position: vec3f,        // 7
	pub _padding: f32,          // 8
	pub tex_coords_off: vec2f,  // 10
	pub tex_coords_size: vec2f, // 12
	pub scale: f32,             // 13
	pub rotation: f32,          // 14
	pub _padding2: [f32; 2],    // 16
}

impl InstanceData {
	pub fn new(position: vec2f, sprite: Sprite) -> Self {
		let (tex_coords_off, tex_coords_size) = index_atlas(sprite);
		Self {
			mix_color: vec4f::default(),
			position: position.append(0.0),
			scale: 1.0,
			rotation: 0.0,
			tex_coords_off,
			tex_coords_size,
			_padding: default(),
			_padding2: default(),
		}
	}

	#[must_use = "does not modify original"]
	pub fn mix_color(mut self, color: vec4f) -> Self{
		self.mix_color = color;
		self
	}
}

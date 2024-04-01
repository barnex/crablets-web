use crate::prelude::*;

/// Uniform data passed to shaders.
///
/// ! `repr(C)` required by WGPU.
/// ! Must be kept in sync with shaders.
/// ! Fields must be aligned to WGPU requirements
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Default, Reflect)]
pub struct UniformData {
	pub camera: mat4x4f,
	//pub angle: f32,
	//pub _padding1: [f32; 3], // be still, my wgpu alignment
}

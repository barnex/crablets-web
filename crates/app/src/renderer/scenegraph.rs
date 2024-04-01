use crate::prelude::*;

#[derive(Clone, Reflect)]
pub struct Scenegraph {
	pub uniforms: UniformData,
	pub instances: Vec<InstanceData>,
	// draw instances back-to-front, starting a new layer at each of these indices.
	pub layer_boundaries: Vec<u32>,
}

impl Scenegraph {
	const MAX_LAYERS: usize = 100;

	pub fn clear(&mut self) {
		self.instances.clear();
		self.layer_boundaries.clear();
	}

	// Start a new layer, drawing over the instances added to the previous layer.
	pub fn new_layer(&mut self) {
		// Each layer issues a draw call, which is quite expensive.
		// Catch accidentally calling new_layer too often.
		debug_assert!(self.layer_boundaries.len() < Self::MAX_LAYERS);

		if !self.instances.is_empty() {
			self.layer_boundaries.push(self.instances.len() as u32)
		}
	}
}

impl Default for Scenegraph {
	fn default() -> Self {
		Self {
			uniforms: UniformData::default(),
			instances: default(),
			layer_boundaries: default(),
		}
	}
}

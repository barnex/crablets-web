use crate::prelude::*;

pub struct Renderer {
	device: Arc<wgpu::Device>,
	queue: Arc<wgpu::Queue>,
	shader_pack: ShaderPack,

	uniform_buffer: UniformBuffer<UniformData>,
	instance_buffer: StorageBuffer<InstanceData>,
	// draw instances from instance buffer layer-by-layer, back to front.
	layer_boundaries: SmallVec<[u32; 4]>,

	texture_atlas: Texture,
}

pub const RED: vec4<u8> = vec4(255, 0, 0, 255);

impl Renderer {
	// Fixed instance buffer byte size. Instances that don't fit won't be rendered.
	const INSTANCE_BUFFER_BYTES: u64 = 4 * 1024 * 1024;

	pub fn new(device: &Arc<wgpu::Device>, queue: &Arc<wgpu::Queue>, target_format: wgpu::TextureFormat, opts: &GraphicsOpts) -> Self {
		Self {
			device: device.clone(),
			queue: queue.clone(),
			shader_pack: ShaderPack::new(device, target_format, opts),
			uniform_buffer: UniformBuffer::new(device, UniformData::default()),
			instance_buffer: StorageBuffer::<InstanceData>::new(device, Self::INSTANCE_BUFFER_BYTES),
			texture_atlas: load_embedded_atlas(device, queue).log_err().unwrap_or_else(|_| Texture::uniform(device, queue, RED)),
			layer_boundaries: default(),
		}
	}

	pub fn prepare(&mut self, _encoder: &mut wgpu::CommandEncoder, sg: &Scenegraph) -> Vec<wgpu::CommandBuffer> {
		self.uniform_buffer.upload(&self.queue, &sg.uniforms);
		self.instance_buffer.upload(&self.device, &self.queue, &sg.instances);

		self.layer_boundaries.clear();
		self.layer_boundaries.extend_from_slice(&sg.layer_boundaries);
		self.layer_boundaries.push(sg.instances.len() as u32);

		vec![]
	}

	pub fn paint<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
		render_pass.set_bind_group(ShaderPack::UNIFORM_GROUP, self.uniform_buffer.binding(), &[]); // bind  @group(0)
		render_pass.set_bind_group(ShaderPack::INSTANCE_GROUP, self.instance_buffer.binding(), &[]); // bind @group(1)
		render_pass.set_bind_group(ShaderPack::TEXTURE_GROUP, self.texture_atlas.binding(), &[]); // bind @group(1)

		render_pass.set_pipeline(&self.shader_pack.quads_pipeline);
		for (start, end) in iter::once(0).chain(self.layer_boundaries.iter().copied()).tuple_windows(){
			// ☠️ TODO: don't draw if end > buffer len!
			render_pass.draw(0..6 /* 2 triangles */, start..end);
		}
	}
}

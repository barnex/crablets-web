use crate::prelude::*;

/// GPU counterpart of `Vec<T>`: a growable buffer of elements,
/// corresponding to `ShaderPack::INSTANCE_LAYOUT`.
///
/// 🪲 Known bug: when close to capacity, does not upload all data.
///
pub struct StorageBuffer<T> {
	buffer: wgpu::Buffer,
	binding: wgpu::BindGroup, // binds buffer to `@binding(0)`, where `ShaderPack` expects it.
	_phantom: PhantomData<T>,
}

impl<T> StorageBuffer<T>
where
	T: Pod + Default,
{
	pub fn new(device: &wgpu::Device, bytes: u64) -> Self {
		let buffer = Self::new_buffer(device, bytes);
		let binding = Self::make_bind_group(device, &buffer);
		Self {
			buffer,
			binding,
			_phantom: PhantomData,
		}
	}

	/// Uploads `data` to the GPU, overwriting previous content.
	/// Sets the buffer's `len`.
	pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
		let bytes = bytemuck::cast_slice(data);
		let mut n = bytes.len();
		if bytes.len() as u64 > self.buffer.size() {
			log::error!("instance buffer full");
			n = self.buffer.size() as usize;
		}

		//log::info!(
		//	"upload {} instances of size {}B, to buffer of {} bytes",
		//	data.len(),
		//	mem::size_of::<T>(),
		//	self.buffer.size()
		//);

		queue.write_buffer(&self.buffer, 0 /*offset*/, &bytes[..n]);
	}

	/// Binds this buffer to @binding(0) (corresponding to `ShaderPack::INSTANCE_LAYOUT`).
	pub fn binding(&self) -> &wgpu::BindGroup {
		&self.binding
	}

	fn new_buffer(device: &wgpu::Device, bytes: u64) -> wgpu::Buffer {
		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some(file!()),
			size: bytes,
			mapped_at_creation: false,
			usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
		})
	}

	fn make_bind_group(device: &wgpu::Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
		let bind_group_layout = device.create_bind_group_layout(&ShaderPack::INSTANCE_LAYOUT);
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some(file!()),
			layout: &bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		})
	}
}

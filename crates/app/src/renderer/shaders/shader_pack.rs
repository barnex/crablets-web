use crate::prelude::*;

/// Collection of mutually compatible shaders (same bind group layouts etc).
pub(crate) struct ShaderPack {
	pub custom3d_pipeline: wgpu::RenderPipeline, // custom3d_wgpu_shader.wgsl
	pub quads_pipeline: wgpu::RenderPipeline,    // quads.wgsl
}

impl ShaderPack {
	/// All shaders in the `ShaderPack` bind their uniforms under group 0.
	pub const UNIFORM_GROUP: u32 = 0;

	/// All shaders in the `ShaderPack` bind their instance data array under group 1.
	pub const INSTANCE_GROUP: u32 = 1;

	/// All shaders in the `ShaderPack` bind their textures+samples under group 2.
	pub const TEXTURE_GROUP: u32 = 2;

	pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat, opts: &GraphicsOpts) -> Self {
		Self {
			custom3d_pipeline: Self::make_shader_pipeline(device, "custom3d_wgpu_shader", include_str!("custom3d_wgpu_shader.wgsl"), target_format),
			quads_pipeline: Self::make_shader_pipeline(device, "quads_shader", include_str!("quads.wgsl"), target_format),
		}
	}

	fn make_shader_pipeline(device: &wgpu::Device, label: &'static str, source: &str, target_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some(label),
			source: wgpu::ShaderSource::Wgsl(source.into()),
		});

		let bind_group_layouts = Self::make_bind_group_layouts(device);

		device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some(label),

			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some(label),
				bind_group_layouts: &[&bind_group_layouts[0], &bind_group_layouts[1], &bind_group_layouts[2]], // 👈 use [T;N].each_ref() when stable
				push_constant_ranges: &[],
			})),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: target_format.remove_srgb_suffix(),
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::OVER,
						alpha: wgpu::BlendComponent::OVER,
					}),
					write_mask: default(),
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			depth_stencil: None,
			//depth_stencil: Some(wgpu::DepthStencilState { 👈 enable depth stencil in main.rs
			//	format: wgpu::TextureFormat::Depth32Float,
			//	depth_write_enabled: true,
			//	depth_compare: wgpu::CompareFunction::Less,
			//	stencil: wgpu::StencilState::default(),
			//	bias: wgpu::DepthBiasState::default(),
			//}),
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		})
	}

	/// Layout of the uniform buffer, identical for all shaders in the `ShaderPack`.
	pub const UNIFORM_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some("ShaderPack::UNIFORM_LAYOUT"),
		entries: &[wgpu::BindGroupLayoutEntry {
			binding: 0,
			visibility: wgpu::ShaderStages::VERTEX,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Uniform,
				has_dynamic_offset: false,
				min_binding_size: None,
			},
			count: None,
		}],
	};

	pub const INSTANCE_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some(file!()),
		entries: &[wgpu::BindGroupLayoutEntry {
			binding: 0,
			visibility: wgpu::ShaderStages::VERTEX,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Storage { read_only: true },
				has_dynamic_offset: false,
				min_binding_size: None,
			},
			count: None,
		}],
	};

	pub const TEXTURE_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some(file!()),
		entries: &[
			// Binding 0: texture data
			wgpu::BindGroupLayoutEntry {
				binding: 0, // shader: @binding(0)
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Texture {
					multisampled: false,
					view_dimension: wgpu::TextureViewDimension::D2,
					sample_type: wgpu::TextureSampleType::Float { filterable: true },
				},
				count: None,
			},
			// Binding 1: texture sampler
			wgpu::BindGroupLayoutEntry {
				binding: 1, // shader: @binding(1)
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
				count: None,
			},
		],
	};

	fn make_bind_group_layouts(device: &wgpu::Device) -> [wgpu::BindGroupLayout; 3] {
		[
			device.create_bind_group_layout(&ShaderPack::UNIFORM_LAYOUT),  // shader: @group(0)
			device.create_bind_group_layout(&ShaderPack::INSTANCE_LAYOUT), // shader: @group(1)
			device.create_bind_group_layout(&ShaderPack::TEXTURE_LAYOUT),  // shader: @group(2)
		]
	}
}

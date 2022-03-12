use num::traits::AsPrimitive;
use num::Float;
use std::marker::PhantomData;
use vek::ColorComponent;
use wgpu::util::DeviceExt;
use yew_wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
];
const INDICES: &[u16] = &[0, 2, 1, 1, 2, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorSliderTruckUniform {
    pub color_start: [f32; 4],
    pub color_end: [f32; 4],
    pub resolution: [f32; 2],
    pub linear: u32, // boolean
}
impl Default for ColorSliderTruckUniform {
    fn default() -> Self {
        Self {
            color_start: [0.0, 0.0, 0.0, 0.0],
            color_end: [0.0, 0.0, 0.0, 0.0],
            resolution: [1.0, 1.0],
            linear: 1,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ColorSliderTruckProps<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod> {
    pub color_start: vek::Rgba<T>,
    pub color_end: vek::Rgba<T>,
    pub linear: bool,
}
impl<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod> Default
    for ColorSliderTruckProps<T>
{
    fn default() -> Self {
        Self {
            color_start: vek::Rgba::black(),
            color_end: vek::Rgba::white(),
            linear: true,
        }
    }
}

pub struct ColorSliderTruckApp<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod> {
    _instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: WgpuCanvasSize,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,

    uniform: ColorSliderTruckUniform,

    _marker: PhantomData<fn() -> T>,
}
impl<T: Float + AsPrimitive<f32> + ColorComponent + bytemuck::Pod> WgpuCanvasApp
    for ColorSliderTruckApp<T>
{
    type Props = ColorSliderTruckProps<T>;

    fn new(canvas_window: WgpuCanvasWindow) -> WgpuCanvasAppCreator<Self> {
        WgpuCanvasAppCreator::new(async move {
            let size = *canvas_window.size();

            let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU);
            let surface = unsafe { instance.create_surface(&canvas_window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap_or_else(|| panic!("Failed to request adapter."));

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None, // Trace path
                )
                .await
                .unwrap_or_else(|_| panic!("Failed to request device."));

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&device, &config);

            let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("color_slider_truck_app.wgsl").into(),
                ),
            });

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

            let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[ColorSliderTruckUniform::default()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("bind_group_layout"),
                });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
                label: Some("bind_group"),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            Self {
                _instance: instance,
                surface,
                device,
                queue,
                config,
                size,

                vertex_buffer,
                index_buffer,
                uniform_buffer,
                bind_group,
                render_pipeline,

                uniform: ColorSliderTruckUniform::default(),

                _marker: PhantomData,
            }
        })
    }

    fn update(&mut self, _delta_time: f64, size: &WgpuCanvasSize) {
        if size.width > 0 && size.height > 0 {
            self.size = *size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
            self.uniform.resolution = [size.width as f32, size.height as f32];
        }

        let output = self.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn update_props(&mut self, props: &Self::Props) {
        self.uniform.color_start = props.color_start.map(|x| x.as_()).into_array();
        self.uniform.color_end = props.color_end.map(|x| x.as_()).into_array();
        self.uniform.linear = if props.linear { 1 } else { 0 };
    }
}

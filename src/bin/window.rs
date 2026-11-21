use glam::Vec3 as GlamVec3;
use jellyfish::dynamics::{Lorenz, generate_trajectory, rk4_step};
use jellyfish::graphics::camera::OrbitCamera;
use jellyfish::graphics::camera_uniform::CameraUniform;
use jellyfish::graphics::projection::normalize_xyz;
use jellyfish::graphics::vertex::Vertex;
use jellyfish::math::Vec3;
use std::sync::Arc;
use std::time::Instant;
use wgpu::CurrentSurfaceTexture::{Suboptimal, Success};
use wgpu::PowerPreference::HighPerformance;
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    instance: Option<wgpu::Instance>,
    surface: Option<wgpu::Surface<'static>>,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
    last_frame: Option<Instant>,
    fps_elapsed: f64,
    fps_frames: u32,
    vertex_buffer: Option<wgpu::Buffer>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_count: u32,
    camera: Option<OrbitCamera>,
    camera_buffer: Option<wgpu::Buffer>,
    camera_bind_group_layout: Option<wgpu::BindGroupLayout>,
    camera_bind_group: Option<wgpu::BindGroup>,
    mouse_pressed: bool,
    last_cursor_position: Option<PhysicalPosition<f64>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("Jellyfish")
            .with_inner_size(winit::dpi::LogicalSize::new(1720.0, 960.0));
        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap(),
        ));
        self.instance = Some(wgpu::Instance::default());

        let window = self.window.as_ref().unwrap();

        let instance = self.instance.as_ref().unwrap();

        self.surface = Some(instance.create_surface(Arc::clone(window)).unwrap());

        let options = wgpu::RequestAdapterOptions {
            power_preference: HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: self.surface.as_ref(),
            apply_limit_buckets: false,
        };

        let future = instance.request_adapter(&options);
        let result = pollster::block_on(future);
        self.adapter = Some(result.unwrap());

        let adapter = self.adapter.as_ref().unwrap();
        let info = adapter.get_info();
        println!("name: {}", info.name);
        println!("backend {:?}", info.backend);
        println!("device: {:?}", info.device_type);

        let descriptor = wgpu::DeviceDescriptor::default();

        let future = adapter.request_device(&descriptor);

        let (device, queue) = pollster::block_on(future).unwrap();
        self.device = Some(device);
        self.queue = Some(queue);

        let surface = self.surface.as_ref().unwrap();
        let size = window.inner_size();
        let config = surface
            .get_default_config(adapter, size.width, size.height)
            .unwrap();

        println!("surface size: {}x{}", config.width, config.height);
        println!("surface format: {:?}", config.format);
        println!("mode: {:?}", config.present_mode);

        let device = self.device.as_ref().unwrap();
        surface.configure(device, &config);
        println!("surface configured success");

        self.config = Some(config);

        let aspect = size.width as f32 / size.height as f32;

        let camera = OrbitCamera::new(GlamVec3::ZERO, 3.0);

        let camera_uniform = CameraUniform::new(&camera, aspect);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("jellyfish camera buffer"),
            contents: bytemuck::bytes_of(&camera_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        self.camera = Some(camera);
        self.camera_buffer = Some(camera_buffer);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("jellyfish camer bind group layout"),
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
            });

        let camera_buffer = self.camera_buffer.as_ref().unwrap();
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("jellyfish camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        self.camera_bind_group_layout = Some(camera_bind_group_layout);
        self.camera_bind_group = Some(camera_bind_group);

        let system = Lorenz {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        };

        let initial = Vec3::new([1.0, 1.0, 1.0]);
        let trajectory = generate_trajectory(&system, initial, 0.01, 10_000, 1000, rk4_step);

        let points: Vec<[f64; 3]> = trajectory.iter().map(|point| *point.as_array()).collect();
        let vertices = normalize_xyz(&points);

        self.vertex_count = vertices.len().try_into().unwrap();

        let buffer_descriptor = wgpu::util::BufferInitDescriptor {
            label: Some("Jellyfish vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        };

        let vertex_buffer = device.create_buffer_init(&buffer_descriptor);

        self.vertex_buffer = Some(vertex_buffer);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Jellyfish line shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../graphics/shaders/line.wgsl").into()),
        });

        let vertex_layouts = [Some(Vertex::layout())];

        let color_targets = [Some(wgpu::ColorTargetState {
            format: self.config.as_ref().unwrap().format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let camera_layout = self.camera_bind_group_layout.as_ref().unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("jellyfish pipeline layout"),
            bind_group_layouts: &[Some(camera_layout)],
            immediate_size: 0,
        });

        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Jellyfish line pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &vertex_layouts,
            },

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                ..Default::default()
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &color_targets,
            }),

            multiview_mask: None,
            cache: None,
        };

        let render_pipeline = device.create_render_pipeline(&pipeline_descriptor);
        self.render_pipeline = Some(render_pipeline);

        // redraw
        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.render();
            }

            WindowEvent::Resized(size) => {
                self.resize(size);
            }

            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_pressed = state == ElementState::Pressed;
                self.last_cursor_position = None;
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed {
                    if let Some(previous) = self.last_cursor_position {
                        let delta_x = (position.x - previous.x) as f32;
                        let delta_y = (position.y - previous.y) as f32;

                        if let Some(camera) = self.camera.as_mut() {
                            camera.rotate(delta_x, delta_y);
                        }

                        self.update_camera_uniform();
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    self.last_cursor_position = Some(position);
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(postition) => postition.y as f32 / 100.0,
                };

                if let Some(camera) = self.camera.as_mut() {
                    camera.zoom(scroll_y);
                    self.update_camera_uniform();
                    self.window.as_ref().unwrap().request_redraw();
                }
            }

            WindowEvent::CursorLeft { .. } => {
                self.last_cursor_position = None;
            }

            WindowEvent::Focused(false) => {
                self.mouse_pressed = false;
                self.last_cursor_position = None;
            }
            _ => (),
        }
    }
}

impl App {
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        let config = self.config.as_mut().unwrap();
        let device = self.device.as_ref().unwrap();
        let surface = self.surface.as_ref().unwrap();
        config.width = size.width;
        config.height = size.height;
        surface.configure(device, config);
        self.update_camera_uniform();
        println!("window resized: {}x{}", size.width, size.height);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn render(&mut self) {
        // println!("Redraw requested");
        let now = Instant::now();

        let delta_time = match self.last_frame {
            Some(previous) => now.duration_since(previous).as_secs_f64(),
            None => 0.0,
        };

        self.last_frame = Some(now);

        self.fps_elapsed += delta_time;

        if self.fps_elapsed >= 1.0 {
            let fps = self.fps_frames as f64 / self.fps_elapsed;
            println!("fps: {:.1}", fps);
            self.fps_elapsed = 0.0;
            self.fps_frames = 0;
        }

        let current_texture = (self.surface.as_ref().unwrap()).get_current_texture();

        let needs_configure = matches!(&current_texture, Suboptimal(_));

        match current_texture {
            Success(texture) | Suboptimal(texture) => {
                // println!("frame acquired");
                let view = texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let device = self.device.as_ref().unwrap();

                let descriptor = wgpu::CommandEncoderDescriptor {
                    label: Some("jellyfish encoder"),
                };
                let mut encoder = device.create_command_encoder(&descriptor);

                let color_attachment = wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.03,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                };

                let color_attachments = [Some(color_attachment)];

                let render_pass_descriptor = wgpu::RenderPassDescriptor {
                    label: Some("Jellyfish render pass"),
                    color_attachments: &color_attachments,
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                };

                {
                    let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);

                    let pipeline = self.render_pipeline.as_ref().unwrap();

                    let vertex_buffer = self.vertex_buffer.as_ref().unwrap();

                    let camera_bind_group = self.camera_bind_group.as_ref().unwrap();

                    render_pass.set_pipeline(pipeline);
                    render_pass.set_bind_group(0, camera_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    // draw
                    render_pass.draw(0..self.vertex_count, 0..1);
                }

                let command_buffer = encoder.finish();
                let queue = self.queue.as_ref().unwrap();
                queue.submit([command_buffer]);
                queue.present(texture);

                self.fps_frames += 1;

                if needs_configure {
                    let size = self.window.as_ref().unwrap().inner_size();
                    self.resize(size);
                } else {
                    self.window.as_ref().unwrap().request_redraw();
                }
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                println!("surface outdated, reconfiguring...");
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();
                self.resize(size);
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                println!("surface lost, recreating...");
                self.recreate_surface();
            }

            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                println!("frame skipped: timeout or occluded");
            }

            other => {
                println!("surface status {:?}", other);
            }
        }
    }

    fn update_camera_uniform(&self) {
        let camera = self.camera.as_ref().unwrap();
        let config = self.config.as_ref().unwrap();
        if config.width == 0 || config.height == 0 {
            return;
        }

        let aspect = config.width as f32 / config.height as f32;
        let uniform = CameraUniform::new(camera, aspect);
        let queue = self.queue.as_ref().unwrap();
        let camera_buffer = self.camera_buffer.as_ref().unwrap();

        queue.write_buffer(camera_buffer, 0, bytemuck::bytes_of(&uniform));
    }

    fn recreate_surface(&mut self) {
        let window = self.window.as_ref().unwrap();
        let instance = self.instance.as_ref().unwrap();

        let size = window.inner_size();

        if size.width == 0 || size.height == 0 {
            return;
        }
        self.surface = None;
        let new_surface = instance.create_surface(Arc::clone(window)).unwrap();
        self.surface = Some(new_surface);
        self.resize(size);
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

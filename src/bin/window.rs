use std::sync::Arc;
use wgpu::CurrentSurfaceTexture::{Suboptimal, Success};
use wgpu::PowerPreference::HighPerformance;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
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

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                let current_texture = (self.surface.as_ref().unwrap()).get_current_texture();

                match current_texture {
                    Success(texture) | Suboptimal(texture) => {
                        println!("frame acquired");
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
                            let _render_pass = encoder.begin_render_pass(&render_pass_descriptor);
                        }

                        let command_buffer = encoder.finish();
                        let queue = self.queue.as_ref().unwrap();
                        queue.submit([command_buffer]);
                        queue.present(texture);
                    }

                    other => {
                        println!("surface status {:?}", other);
                    }
                }
            }

            WindowEvent::Resized(size) => {
                self.resize(size);
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
        println!("window resized: {}x{}", size.width, size.height);
        self.window.as_ref().unwrap().request_redraw();
    }
}


fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

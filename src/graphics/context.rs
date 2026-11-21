use crate::graphics::render::Render;
use std::sync::Arc;
use wgpu::CurrentSurfaceTexture::{Suboptimal, Success};
use wgpu::PowerPreference::HighPerformance;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub enum FrameStatus {
    Presented { reconfigured: bool },
    Reconfigured,
    Skipped,
}

pub struct GpuContext {
    instance: wgpu::Instance,
    surface: Option<wgpu::Surface<'static>>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let options = wgpu::RequestAdapterOptions {
            power_preference: HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            apply_limit_buckets: false,
        };

        let adapter = pollster::block_on(instance.request_adapter(&options)).unwrap();

        let info = adapter.get_info();

        println!("name: {}", info.name);
        println!("backend {:?}", info.backend);
        println!("device: {:?}", info.device_type);

        let descriptor = wgpu::DeviceDescriptor::default();

        let (device, queue) = pollster::block_on(adapter.request_device(&descriptor)).unwrap();

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        println!("surface size: {}x{}", config.width, config.height);
        println!("surface format: {:?}", config.format);
        println!("mode: {:?}", config.present_mode);

        surface.configure(&device, &config);

        println!("surface configured success");

        Self {
            instance,
            surface: Some(surface),
            device,
            queue,
            config,
        }
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    pub fn aspect(&self) -> f32 {
        self.config.width as f32 / self.config.height as f32
    }

    pub fn height(&self) -> f32 {
        self.config.height as f32
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> bool {
        if size.width == 0 || size.height == 0 {
            return false;
        }

        self.config.width = size.width;
        self.config.height = size.height;

        self.surface
            .as_ref()
            .unwrap()
            .configure(&self.device, &self.config);

        true
    }

    pub fn render(&mut self, renderer: &Render, window: &Arc<Window>) -> FrameStatus {
        let current_texture = self.surface.as_ref().unwrap().get_current_texture();

        let needs_configure = matches!(&current_texture, Suboptimal(_));

        match current_texture {
            Success(texture) | Suboptimal(texture) => {
                let view = texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                renderer.draw_frame(&self.device, &self.queue, &view);

                self.queue.present(texture);

                if needs_configure && self.resize(window.inner_size()) {
                    FrameStatus::Presented { reconfigured: true }
                } else {
                    FrameStatus::Presented {
                        reconfigured: false,
                    }
                }
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                println!("surface outdated, reconfiguring...");

                if self.resize(window.inner_size()) {
                    FrameStatus::Reconfigured
                } else {
                    FrameStatus::Skipped
                }
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                println!("surface lost, recreating...");

                if self.recreate_surface(Arc::clone(window)) {
                    FrameStatus::Reconfigured
                } else {
                    FrameStatus::Skipped
                }
            }

            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                println!("frame skipped: timeout or occluded");
                FrameStatus::Skipped
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                println!("surface validation error");
                FrameStatus::Skipped
            }
        }
    }

    fn recreate_surface(&mut self, window: Arc<Window>) -> bool {
        let size = window.inner_size();

        if size.width == 0 || size.height == 0 {
            return false;
        }

        self.surface = None;

        let new_surface = self.instance.create_surface(window).unwrap();

        self.surface = Some(new_surface);

        self.resize(size)
    }
}

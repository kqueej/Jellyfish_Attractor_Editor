use crate::dynamics::Attractor;
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use winit::event::WindowEvent;
use winit::window::Window;
use super::panel;

pub struct Gui {
    context: egui::Context,
    state: egui_winit::State,
    renderer: Renderer,
}

impl Gui {
    pub fn new(
        window: &Window,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) -> Self {
        let context = egui::Context::default();

        let state = egui_winit::State::new(
            context.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let renderer = Renderer::new(
            device,
            format,
            RendererOptions::default(),
        );

        Self {
            context,
            state,
            renderer,
        }
    }

    pub fn on_window_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) -> egui_winit::EventResponse {
        self.state.on_window_event(window, event)
    }

    pub fn draw(
        &mut self,
        window: &Window,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        attractor: &mut Attractor,
    ) -> bool {
        let input = self.state.take_egui_input(window);

        let mut parameters_changed = false;

        let output = self.context.run_ui(input, |ui| {
            parameters_changed |= panel::show(ui, attractor);
        });
        self.state
            .handle_platform_output(window, output.platform_output);

        let paint_jobs = self.context.tessellate(
            output.shapes,
            output.pixels_per_point,
        );

        let size = window.inner_size();

        let screen = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: output.pixels_per_point,
        };


        for (id, deltas) in &output.textures_delta.set {
            for delta in deltas {
                self.renderer.update_texture(
                    device,
                    queue,
                    *id,
                    delta,
                );
            }
}
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Jellyfish GUI encoder"),
            });

        let extra_commands = self.renderer.update_buffers(
            device,
            queue,
            &mut encoder,
            &paint_jobs,
            &screen,
        );

        {
            let color_attachment = wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            };

            let attachments = [Some(color_attachment)];

            let mut pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Jellyfish GUI pass"),
                    color_attachments: &attachments,
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                })
                .forget_lifetime();

            self.renderer.render(
                &mut pass,
                &paint_jobs,
                &screen,
            );
        }

        queue.submit(
            extra_commands
                .into_iter()
                .chain(std::iter::once(encoder.finish())),
        );

        for id in &output.textures_delta.free {
            self.renderer.free_texture(id);
        }
        parameters_changed
    }
}
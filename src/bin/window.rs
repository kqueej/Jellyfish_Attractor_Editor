use glam::Vec3 as GlamVec3;
use jellyfish::animation::AnimationController;
use jellyfish::graphics::camera::OrbitCamera;
use jellyfish::graphics::context::{FrameStatus, GpuContext};
use jellyfish::graphics::render::Render;
use jellyfish::scene::config::AttractorConfig;
use jellyfish::ui::Gui;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    last_frame: Option<Instant>,
    fps_elapsed: f64,
    fps_frames: u32,
    renderer: Option<Render>,
    camera: Option<OrbitCamera>,
    mouse_pressed: bool,
    last_cursor_position: Option<PhysicalPosition<f64>>,
    pan_pressed: bool,
    animation: Option<AnimationController>,
    attractor_config: Option<AttractorConfig>,
    gui: Option<Gui>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let window_attributes = Window::default_attributes()
            .with_title("Jellyfish")
            .with_inner_size(winit::dpi::LogicalSize::new(1720.0, 960.0));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let gpu = GpuContext::new(Arc::clone(&window));
        let gui = Gui::new(
            window.as_ref(),
            gpu.device(),
            gpu.format(),
        );

        let camera = OrbitCamera::new(GlamVec3::ZERO, 3.0);
        let attractor_config = AttractorConfig::lorenz();
        let vertices = attractor_config.vertices();

        self.animation = Some(AnimationController::new(vertices.len().try_into().unwrap()));

        let renderer = Render::new(gpu.device(), gpu.format(), &camera, gpu.aspect(), &vertices);
        self.window = Some(Arc::clone(&window));
        self.gpu = Some(gpu);
        self.camera = Some(camera);
        self.renderer = Some(renderer);
        self.attractor_config = Some(attractor_config);
        self.gui = Some(gui);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {


        let egui_consumed = if let (Some(gui), Some(window)) =
            (self.gui.as_mut(), self.window.as_ref())
        {
            let response = gui.on_window_event(window, &event);

            if response.repaint {
                window.request_redraw();
            }

            response.consumed
        } else {
            false
        };

        if egui_consumed {
            let is_pointer_event = matches!(
                &event,
                WindowEvent::MouseInput { .. }
                    | WindowEvent::MouseWheel { .. }
                    | WindowEvent::CursorMoved { .. }
            );

            if is_pointer_event {
                self.mouse_pressed = false;
                self.pan_pressed = false;
                self.last_cursor_position = None;
            }

            if is_pointer_event
                || matches!(&event, WindowEvent::KeyboardInput { .. })
            {
                return;
            }
        }

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

            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::Space) => {
                            self.animation.as_mut().unwrap().toggle_pause();
                        }

                        PhysicalKey::Code(KeyCode::KeyR) => {
                            self.animation.as_mut().unwrap().reset();
                        }

                        PhysicalKey::Code(KeyCode::Digit1) => {
                            self.switch_attractor(AttractorConfig::lorenz());
                        }

                        PhysicalKey::Code(KeyCode::Digit2) => {
                            self.switch_attractor(AttractorConfig::rossler());
                        }
                        _ => {}
                    }

                    match &event.logical_key {
                        Key::Character(key) if key.as_str() == "+" => {
                            let animation = self.animation.as_mut().unwrap();

                            animation.speed_up();

                            println!("Animation speed: {:.0}", animation.speed());
                        }

                        Key::Character(key) if key.as_str() == "-" => {
                            let animation = self.animation.as_mut().unwrap();

                            animation.slow_down();

                            println!("Animation speed: {:.0}", animation.speed());
                        }

                        _ => {}
                    }
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed || self.pan_pressed {
                    if let Some(previous) = self.last_cursor_position {
                        let delta_x = (position.x - previous.x) as f32;
                        let delta_y = (position.y - previous.y) as f32;

                        if let Some(camera) = self.camera.as_mut() {
                            if self.pan_pressed {
                                let height = self.gpu.as_ref().unwrap().height();
                                camera.pan(delta_x, delta_y, height);
                            } else {
                                camera.rotate(delta_x, delta_y);
                            }
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
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 100.0,
                };

                if let Some(camera) = self.camera.as_mut() {
                    camera.zoom(scroll_y);
                    self.update_camera_uniform();
                    self.window.as_ref().unwrap().request_redraw();
                }
            }

            WindowEvent::MouseInput {
                state,
                button: MouseButton::Right,
                ..
            } => {
                self.pan_pressed = state == ElementState::Pressed;
                self.last_cursor_position = None;
            }

            WindowEvent::CursorLeft { .. } => {
                self.last_cursor_position = None;
            }

            WindowEvent::Focused(false) => {
                self.mouse_pressed = false;
                self.pan_pressed = false;
                self.last_cursor_position = None;
            }
            _ => (),
        }
    }
}

impl App {
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if self.gpu.as_mut().unwrap().resize(size) {
            self.update_camera_uniform();

            println!("window resized: {}x{}", size.width, size.height);

            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn render(&mut self) {
        let now = Instant::now();

        let delta_time = match self.last_frame {
            Some(previous) => now.duration_since(previous).as_secs_f64(),
            None => 0.0,
        };

        self.animation.as_mut().unwrap().update(delta_time);

        self.last_frame = Some(now);
        self.fps_elapsed += delta_time;

        if self.fps_elapsed >= 1.0 {
            let fps = self.fps_frames as f64 / self.fps_elapsed;

            println!("fps: {:.1}", fps);

            self.fps_elapsed = 0.0;
            self.fps_frames = 0;
        }



        let window = self.window.as_ref().unwrap();
        let renderer = self.renderer.as_ref().unwrap();
        let gui = self.gui.as_mut().unwrap();

        let attractor = &mut self
            .attractor_config
            .as_mut()
            .unwrap()
            .system;

        let visible_vertices = self
            .animation
            .as_ref()
            .unwrap()
            .visible_vertices();

        let mut parameters_changed = false;

        let result = self.gpu.as_mut().unwrap().render_with_overlay(
            renderer,
            window,
            visible_vertices,
            |device, queue, view| {
                parameters_changed =
                    gui.draw(window, device, queue, view, attractor);
            },
        );

        if parameters_changed {
            self.rebuild_attractor_vertices();
        }


        match result {
            FrameStatus::Presented { reconfigured } => {
                self.fps_frames += 1;

                if reconfigured {
                    self.update_camera_uniform();
                }

                self.window.as_ref().unwrap().request_redraw();
            }

            FrameStatus::Reconfigured => {
                self.update_camera_uniform();
                self.window.as_ref().unwrap().request_redraw();
            }

            FrameStatus::Skipped => {}
        }
    }

    fn update_camera_uniform(&self) {
        let gpu = self.gpu.as_ref().unwrap();
        let camera = self.camera.as_ref().unwrap();

        self.renderer
            .as_ref()
            .unwrap()
            .update_camera(gpu.queue(), camera, gpu.aspect());
    }

    fn switch_attractor(&mut self, config: AttractorConfig) {
        let vertices = config.vertices();

        let Some(gpu) = self.gpu.as_ref() else {
            return;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        renderer.update_vertices(gpu.device(), &vertices);

        let vertex_count = u32::try_from(vertices.len()).expect("Too many vertices");

        self.animation = Some(AnimationController::new(vertex_count));
        self.attractor_config = Some(config);
        self.last_frame = Some(Instant::now());

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }


    fn rebuild_attractor_vertices(&mut self) {
        let Some(config) = self.attractor_config.as_ref() else {
            return;
        };

        let vertices = config.vertices();

        let Some(gpu) = self.gpu.as_ref() else {
            return;
        };

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        renderer.update_vertices(gpu.device(), &vertices);

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

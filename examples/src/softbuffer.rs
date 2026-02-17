use std::{num::NonZeroU32, sync::Arc};

use softbuffer::{Context, Surface};
use tiny_pao::{Canvas, Color, Size};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalSize, Size as WinitSize},
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

fn main() {
    let event_loop = EventLoop::new();
    event_loop.unwrap().run_app(&mut App::default()).unwrap();
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    canvas: Option<Canvas>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.canvas.is_none() && self.window.is_none() && self.surface.is_none() {
            let window_attributes =
                WindowAttributes::default().with_inner_size(WinitSize::Physical(PhysicalSize {
                    width: 600,
                    height: 600,
                }));
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let canvas = Canvas::new(
                Size {
                    width: 600,
                    height: 600,
                },
                Color::rgb(255, 255, 255),
            );
            let surface = {
                let context = Context::new(window.clone()).unwrap();
                Some(Surface::new(&context, window.clone()).unwrap())
            };

            window.request_redraw();
            self.window = Some(window);
            self.surface = surface;
            self.canvas = Some(canvas);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                if let (Some(surface), Some(canvas)) = (&mut self.surface, &mut self.canvas) {
                    canvas.clear(Color::rgb(255, 255, 255));
                    let mut buffer = surface.buffer_mut().unwrap();
                    buffer.copy_from_slice(canvas.buffer());
                    buffer.present().unwrap();
                }
            }
            winit::event::WindowEvent::Resized(size) => {
                if let (Some(window), Some(canvas), Some(surface)) =
                    (self.window.as_ref(), &mut self.canvas, &mut self.surface)
                {
                    canvas.resize(Size {
                        width: size.width,
                        height: size.height,
                    });

                    let _ = surface.resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );

                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

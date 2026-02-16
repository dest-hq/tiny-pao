use std::sync::Arc;

use tiny_pao::{Canvas, color::Color};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalSize, Size},
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
    canvas: Option<Canvas<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.canvas.is_none() && self.window.is_none() {
            let window_attributes =
                WindowAttributes::default().with_inner_size(Size::Physical(PhysicalSize {
                    width: 600,
                    height: 600,
                }));
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let canvas = Canvas::new(600, 600, Color::rgb(255, 255, 255), window.clone());
            window.request_redraw();
            self.window = Some(window);
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
                if let Some(canvas) = &mut self.canvas {
                    canvas.clear(Color::rgb(255, 255, 255));
                    canvas.present().unwrap();
                }
            }
            winit::event::WindowEvent::Resized(size) => {
                if let (Some(window), Some(canvas)) = (self.window.as_ref(), &mut self.canvas) {
                    canvas.resize(size.width, size.height).unwrap();

                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

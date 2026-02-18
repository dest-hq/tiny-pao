// To run the example, execute this command: cargo run --release --bin text_render

use std::{num::NonZeroU32, sync::Arc};

use cosmic_text::{Attrs, Buffer, Color as CosmicColor, FontSystem, Metrics, Shaping, SwashCache};
use softbuffer::{Context, Surface};
use tiny_pao::{Canvas, Color, Position, Size};
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
    font_sys: Option<FontSystem>,
    swash_cache: Option<SwashCache>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.canvas.is_none()
            && self.window.is_none()
            && self.surface.is_none()
            && self.font_sys.is_none()
            && self.swash_cache.is_none()
        {
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
                Color::rgb(0, 0, 0),
            );
            let surface = {
                let context = Context::new(window.clone()).unwrap();
                Some(Surface::new(&context, window.clone()).unwrap())
            };

            let font_system = FontSystem::new();
            let swash_cache = SwashCache::new();

            window.request_redraw();
            self.window = Some(window);
            self.surface = surface;
            self.font_sys = Some(font_system);
            self.swash_cache = Some(swash_cache);
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
                if let (Some(surface), Some(canvas), Some(font_sys), Some(swash_cache)) = (
                    &mut self.surface,
                    &mut self.canvas,
                    &mut self.font_sys,
                    &mut self.swash_cache,
                ) {
                    canvas.clear(Color::rgb(0, 0, 0));

                    // Text metrics indicate the font size and line height of a buffer
                    const FONT_SIZE: f32 = 20.0;
                    const LINE_HEIGHT: f32 = FONT_SIZE * 1.2;
                    let metrics = Metrics::new(FONT_SIZE, LINE_HEIGHT);

                    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
                    let mut buffer = Buffer::new(font_sys, metrics);

                    let mut buffer = buffer.borrow_with(font_sys);

                    // Set a size for the text buffer, in pixels
                    let width = 200.0;
                    // The height is unbounded
                    buffer.set_size(Some(width), None);

                    // Attributes indicate what font to choose
                    let attrs = Attrs::new();

                    // Add some text!
                    let text = std::env::args()
                        .nth(1)
                        .unwrap_or(" Hi, Rust! 🦀 ".to_string());
                    buffer.set_text(&text, &attrs, Shaping::Advanced, None);

                    // Perform shaping as desired
                    buffer.shape_until_scroll(true);

                    // Default text color (0xFF, 0xFF, 0xFF is white)
                    const TEXT_COLOR: CosmicColor = CosmicColor::rgb(0xFF, 0xFF, 0xFF);

                    // Set up the canvas
                    let height = LINE_HEIGHT * buffer.layout_runs().count() as f32;

                    // Draw to the canvas
                    buffer.draw(swash_cache, TEXT_COLOR, |x, y, w, h, color| {
                        let a = color.a();
                        if a == 0
                            || x < 0
                            || x >= width as i32
                            || y < 0
                            || y >= height as i32
                            || w != 1
                            || h != 1
                        {
                            // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                            return;
                        }

                        // Scale by alpha (mimics blending with black)
                        let scale = |c: u8| (c as i32 * a as i32 / 255).clamp(0, 255) as u8;

                        let r = scale(color.r());
                        let g = scale(color.g());
                        let b = scale(color.b());
                        canvas.draw_pixel(
                            Position {
                                x: x as u32,
                                y: y as u32,
                            },
                            Color::rgb(r, g, b),
                        );
                    });

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

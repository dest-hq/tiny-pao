use std::{num::NonZeroU32, sync::Arc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use softbuffer::{Context, SoftBufferError, Surface};

use crate::{Color, Position, Size};

pub struct Canvas<B>
where
    B: HasWindowHandle + HasDisplayHandle,
{
    width: std::num::NonZero<u32>,
    height: std::num::NonZero<u32>,
    buffer: Vec<u32>,
    surface: Option<Surface<Arc<B>, Arc<B>>>,
    color: Color,
}

impl<B> Canvas<B>
where
    B: HasWindowHandle + HasDisplayHandle,
{
    /// Create new canvas
    pub fn new(window_size: Size, color: Color, window: Arc<B>) -> Self {
        let surface = {
            let context = Context::new(window.clone()).unwrap();
            Some(Surface::new(&context, window).unwrap())
        };

        let u32_color = Color::to_argb(color.a, color.r, color.g, color.b);
        Self {
            width: NonZeroU32::new(window_size.width).unwrap(),
            height: NonZeroU32::new(window_size.height).unwrap(),
            buffer: vec![u32_color; (window_size.width * window_size.height) as usize],
            surface: surface,
            color: color,
        }
    }

    pub fn draw_rect(&mut self, pos: Position, size: Size, color: Color) {
        for y in 0..size.height {
            for x in 0..size.width {
                self.draw_pixel(
                    Position {
                        x: x + pos.x,
                        y: y + pos.y,
                    },
                    color,
                );
            }
        }
    }

    pub fn draw_rounded_rect(&mut self, pos: Position, size: Size, color: Color, radius: u32) {
        //
        // Credits:
        // https://mathworld.wolfram.com/RoundedRectangle.html
        // https://mathworld.wolfram.com/Circle.html
        //

        for y in 0..size.height {
            for x in 0..size.width {
                // Distance to the center of circle
                let mut dx = 0;
                let mut dy = 0;

                // Top left
                if x < radius && y < radius {
                    dx = radius;
                    dy = radius;
                }
                // Top right
                else if x > size.width - radius && y < radius {
                    dx = size.width - radius;
                    dy = radius;
                }
                // Bottom left
                else if x < radius && y < size.height - radius {
                    dx = radius;
                    dy = size.height - radius;
                }
                // Bottom right
                else if x > size.width - radius && y < size.height - radius {
                    dx = size.width - radius;
                    dy = size.height - radius;
                }

                if dx + dy <= radius {
                    self.draw_pixel(
                        Position {
                            x: x + pos.x,
                            y: y + pos.y,
                        },
                        color,
                    );
                }
            }
        }
    }

    /// Remove all stuff from canvas
    pub fn clear(&mut self, color: Color) {
        let u32_color = Color::to_argb(color.a, color.r, color.g, color.b);
        self.buffer.fill(u32_color);
    }

    /// Resize the canvas
    pub fn resize(&mut self, size: Size) -> Result<(), SoftBufferError> {
        if let Some(surface) = &mut self.surface {
            let non_width = NonZeroU32::new(size.width).unwrap();
            let non_height = NonZeroU32::new(size.height).unwrap();

            surface.resize(non_width, non_height)?;

            self.width = non_width;
            self.height = non_height;

            let u32_color = Color::to_argb(self.color.a, self.color.r, self.color.g, self.color.b);
            self.buffer = vec![u32_color; (self.width.get() * self.height.get()) as usize];
        }

        Ok(())
    }

    pub fn draw_pixel(&mut self, pixel: Position, color: Color) {
        if pixel.x >= self.width.get() || pixel.y >= self.height.get() {
            return; // prevent out-of-bounds
        }

        let u32_color = Color::to_argb(color.a, color.r, color.g, color.b);

        let index = (pixel.y * self.width.get() + pixel.x) as usize;

        self.buffer[index] = u32_color;
    }

    /// Give the pixels
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    /// Draw
    pub fn present(&mut self) -> Result<(), SoftBufferError> {
        if let Some(surface) = &mut self.surface {
            let mut buffer = surface.buffer_mut()?;
            buffer.copy_from_slice(&self.buffer);
            buffer.present()?;

            return Ok(());
        }
        Err(SoftBufferError::IncompleteDisplayHandle)
    }
}

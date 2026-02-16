use std::{num::NonZeroU32, sync::Arc};

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use softbuffer::{Context, SoftBufferError, Surface};

use crate::core::color::Color;

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
    pub fn new(width: u32, height: u32, color: Color, window: Arc<B>) -> Self {
        let surface = {
            let context = Context::new(window.clone()).unwrap();
            Some(Surface::new(&context, window).unwrap())
        };

        let u32_color = u32::from_be_bytes([color.r, color.g, color.b, color.a]);
        Self {
            width: NonZeroU32::new(width).unwrap(),
            height: NonZeroU32::new(height).unwrap(),
            buffer: vec![u32_color; (width * height) as usize],
            surface: surface,
            color: color,
        }
    }

    /// Remove all stuff from canvas
    pub fn clear(&mut self, color: Color) {
        let u32_color = u32::from_be_bytes([color.r, color.g, color.b, color.a]);
        self.buffer.fill(u32_color);
    }

    /// Resize the canvas
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), SoftBufferError> {
        if let Some(surface) = &mut self.surface {
            let non_width = NonZeroU32::new(width).unwrap();
            let non_height = NonZeroU32::new(height).unwrap();

            surface.resize(non_width, non_height)?;

            self.width = non_width;
            self.height = non_height;

            let u32_color =
                u32::from_be_bytes([self.color.r, self.color.g, self.color.b, self.color.a]);
            self.buffer = vec![u32_color; (self.width.get() * self.height.get()) as usize];
        }

        Ok(())
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

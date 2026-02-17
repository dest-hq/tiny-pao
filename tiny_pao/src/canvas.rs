use crate::{Color, Position, Size};

pub struct Canvas {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    color: Color,
}

impl Canvas {
    /// Create new canvas
    pub fn new(size: Size, color: Color) -> Self {
        let u32_color = Color::to_argb(color.a, color.r, color.g, color.b);
        Self {
            width: size.width,
            height: size.height,
            buffer: vec![u32_color; (size.width * size.height) as usize],
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

    pub fn draw_circle(&mut self, pos: Position, size: Size, color: Color) {
        self.draw_rounded_rect(pos, size, color, 360);
    }

    pub fn draw_rounded_rect(&mut self, pos: Position, size: Size, color: Color, radius: u32) {
        //
        // Credits:
        // https://mathworld.wolfram.com/RoundedRectangle.html
        // https://mathworld.wolfram.com/Circle.html
        //

        // Get the relevant radius for size because there are moments when the radius, like 360, is bigger than the size due to errors
        let max_radius = std::cmp::min(size.width, size.height) / 2;
        let radius = if radius > max_radius {
            max_radius
        } else {
            radius
        };

        for y in 0..size.height {
            for x in 0..size.width {
                // Distance to the center of circle
                let mut dx = 0;
                let mut dy = 0;

                // Top left
                if x < radius && y < radius {
                    dx = radius - x;
                    dy = radius - y;
                }
                // Top right
                else if x >= size.width - radius && y < radius {
                    dx = x - (size.width - radius - 1);
                    dy = radius - y;
                }
                // Bottom left
                else if x < radius && y >= size.height - radius {
                    dx = radius - x;
                    dy = y - (size.height - radius - 1);
                }
                // Bottom right
                else if x >= size.width - radius && y >= size.height - radius {
                    dx = x - (size.width - radius - 1);
                    dy = y - (size.height - radius - 1);
                }

                if dx * dx + dy * dy <= radius * radius {
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
    pub fn resize(&mut self, size: Size) {
        self.width = size.width;
        self.height = size.height;

        let u32_color = Color::to_argb(self.color.a, self.color.r, self.color.g, self.color.b);
        self.buffer = vec![u32_color; (self.width * self.height) as usize];
    }

    pub fn draw_pixel(&mut self, pixel: Position, color: Color) {
        if pixel.x >= self.width || pixel.y >= self.height {
            return; // prevent out-of-bounds
        }

        let u32_color = Color::to_argb(color.a, color.r, color.g, color.b);

        let index = (pixel.y * self.width + pixel.x) as usize;

        self.buffer[index] = u32_color;
    }

    /// Give the pixels
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }
}

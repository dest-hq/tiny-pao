#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: 255,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn to_argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
        ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
}

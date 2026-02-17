use image::{ImageBuffer, Rgba};
use tiny_pao::{Canvas, Color, Position, Size};

fn main() {
    let mut canvas = Canvas::new(
        Size {
            width: 600,
            height: 600,
        },
        Color::rgb(255, 255, 255),
    );

    canvas.draw_circle(
        Position { x: 0, y: 0 },
        Size {
            width: 600,
            height: 600,
        },
        Color::rgb(0, 0, 0),
    );

    let buffer = canvas.buffer();

    let image = create_image_from_vec(buffer, 600, 600);

    image.save("output.png").unwrap();
}

fn create_image_from_vec(
    colors: &[u32],
    width: u32,
    height: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);

    for (i, &color) in colors.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        if x < width && y < height {
            let r = (color >> 16) as u8;
            let g = (color >> 8) as u8;
            let b = color as u8;
            let a = (color >> 24) as u8;
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img
}

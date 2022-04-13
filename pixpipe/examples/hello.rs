use image::ColorType;
use pixpipe::{Color, PixBuf};

fn main() {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 200;
    let mut pix_buf = PixBuf::with_dimensions(WIDTH, HEIGHT);

    pix_buf.fill(Color::DARK_GRAY);

    pix_buf.set(10, 10, Color::RED);
    pix_buf.set(20, 30, Color::GREEN);
    pix_buf.set(30, 20, Color::BLUE);

    image::save_buffer("test.png", pix_buf.as_slice(), WIDTH, HEIGHT, ColorType::Rgba8).unwrap();
}

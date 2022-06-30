use glium::{texture::RawImage2d, uniforms::MagnifySamplerFilter, Display, Surface};

use crate::ppu::palette::Color;

pub fn render(display: &Display, buffer: [Color; 256 * 256]) {
    let target = display.draw();
    // target.clear_color(1.0, 0.0, 0.0, 1.0);

    let color_buffer: Vec<u8> = buffer
        .iter()
        .flat_map(|color| color_to_bytes(color))
        .collect();

    let image = RawImage2d::from_raw_rgb_reversed(&color_buffer, (256, 256));
    let texture = glium::texture::Texture2d::new(display, image).unwrap();

    texture
        .as_surface()
        .fill(&target, MagnifySamplerFilter::Linear);

    target.finish().unwrap();
}

fn color_to_bytes(color: &Color) -> [u8; 3] {
    match color {
        Color::White => [0xff, 0xff, 0xff],
        Color::LightGray => [0xcc, 0xcc, 0xcc],
        Color::DarkGray => [0x77, 0x77, 0x77],
        Color::Black => [0, 0x00, 0x00],
    }
}

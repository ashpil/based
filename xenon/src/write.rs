use std::io::BufWriter;
use std::fs::File;
use crate::color::Color;

pub fn fn_to_png<F>(width: u32, height: u32, file: File, mut func: F) where F: FnMut(u32, u32) -> Color {
    let mut data = Vec::with_capacity((width * height * 6) as usize);
    for j in (1..=height).rev() {
        for i in 0..width {
            write_pixel(&mut data, func(i, j));
        }
    }
    write_to_file(data, file, width, height);
}

pub fn write_pixel(buffer: &mut Vec<u8>, pixel: Color) {
    let red = (pixel.r.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let green = (pixel.g.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let blue = (pixel.b.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let to_write = [
        (red >> 8) as u8, red as u8, 
        (green >> 8) as u8, green as u8, 
        (blue >> 8) as u8, blue as u8, 
    ];
    buffer.extend_from_slice(&to_write);
}

fn write_to_file(data: Vec<u8>, file: File, width: u32, height: u32) {
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Sixteen);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}


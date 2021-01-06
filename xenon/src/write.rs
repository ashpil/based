use std::io::BufWriter;
use std::fs::File;
use crate::color::Color;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub fn fn_to_png(width: u32, height: u32, file: File, func: impl Fn(u32, u32) -> Color + Sync + Send) {
    let mut data = vec![0; (width * height * 6) as usize];
    #[cfg(feature = "rayon")]
    let iter = data.par_chunks_exact_mut(6);
    #[cfg(not(feature = "rayon"))]
    let iter = data.chunks_exact_mut(6);
    iter.enumerate().for_each(|(index, pixel)| {
        let i = (index as u32) % width;
        let j = height - ((index as u32) / width);
        write_pixel(pixel, func(i, j));
    });
    write_to_file(data, file, width, height);
}

pub fn write_pixel(buffer: &mut [u8], pixel: Color) {
    let red = (pixel.r.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let green = (pixel.g.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let blue = (pixel.b.sqrt().clamp(0.0, 0.999) * 65535.0) as u16;
    let mut to_write = [
        (red >> 8) as u8, red as u8, 
        (green >> 8) as u8, green as u8, 
        (blue >> 8) as u8, blue as u8, 
    ];

    buffer.copy_from_slice(&mut to_write);
}

fn write_to_file(data: Vec<u8>, file: File, width: u32, height: u32) {
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Sixteen);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}


use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

fn main() {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 100;

    let path = Path::new(r"test.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = Vec::<u8>::with_capacity((4 * WIDTH * HEIGHT) as usize);
    let d_height = 255 / HEIGHT;
    let d_width = 255 / WIDTH;

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            data.push((d_width * x) as u8);
            data.push((d_height * y) as u8);
            data.push(255 / 5);
            data.push(255);
        }
    }

    writer.write_image_data(&data).unwrap();
}

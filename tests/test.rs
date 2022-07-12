#![feature(test)]

use noilib_simple::NoiseGenerator;

fn generate_image(f: impl Fn(f32, f32) -> f32, image_name: &str) {
    let mut image: image::RgbImage = image::ImageBuffer::new(1024, 1024);
    for (x, y, rgb) in image.enumerate_pixels_mut() {
        let v = f((x as f32 - 512.0) / 64.0, (y as f32 - 512.0) / 64.0);
        let v = (v * 128.0 + 128.0) as u8;
        rgb.0 = [v, v, v];
    }
    image
        .save("test-images/".to_owned() + image_name + ".png")
        .unwrap();
}

#[test]
fn perlin_noise_image() {
    let noise = NoiseGenerator::new(0);
    generate_image(|x, y| noise.perlin(x, y), "perlin_noise");
}
#[test]
fn value_noise_image() {
    let noise = NoiseGenerator::new(0);
    generate_image(|x, y| noise.value(x * 2.0, y * 2.0), "value_noise");
}
#[test]
fn perlin_noise_4_octaves_image() {
    let noise = NoiseGenerator::new(0);
    generate_image(
        |x, y| noise.perlin_octaves(x, y, 4, 0.5),
        "perlin_noise_4_octaves",
    );
}
#[test]
fn value_noise_4_octaves_image() {
    let noise = NoiseGenerator::new(0);
    generate_image(
        |x, y| noise.value_octaves(x * 2.0, y * 2.0, 4, 0.5),
        "value_noise_4_octaves",
    );
}

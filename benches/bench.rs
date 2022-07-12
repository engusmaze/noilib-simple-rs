#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

use noilib_simple::NoiseGenerator;

#[bench]
fn value_noise(b: &mut Bencher) {
    let noise = NoiseGenerator::new(0);
    let mut x = 0.0;
    let mut y = 0.0;
    b.iter(|| {
        x += 0.1;
        y += 0.2;
        black_box(noise.value(x, y));
    });
}
#[bench]
fn perlin_noise(b: &mut Bencher) {
    let noise = NoiseGenerator::new(0);
    let mut x = 0.0;
    let mut y = 0.0;
    b.iter(|| {
        x += 0.1;
        y += 0.2;
        black_box(noise.perlin(x, y));
    });
}
#[bench]
fn value_noise_4_octaves(b: &mut Bencher) {
    let noise = NoiseGenerator::new(0);
    let mut x = 0.0;
    let mut y = 0.0;
    b.iter(|| {
        x += 0.1;
        y += 0.2;
        black_box(noise.value_octaves(x, y, 4, 0.5));
    });
}
#[bench]
fn perlin_noise_4_octaves(b: &mut Bencher) {
    let noise = NoiseGenerator::new(0);
    let mut x = 0.0;
    let mut y = 0.0;
    b.iter(|| {
        x += 0.1;
        y += 0.2;
        black_box(noise.perlin_octaves(x, y, 4, 0.5));
    });
}
#[bench]
fn noise_generator(b: &mut Bencher) {
    let mut seed = 0;
    b.iter(|| {
        seed += 1;
        black_box(NoiseGenerator::new(seed));
    });
}

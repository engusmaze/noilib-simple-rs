use noilib_simple::NoiseGenerator;

fn main() {
    let seed = 123;
    let noise = NoiseGenerator::new(seed); // Initializing noise generator

    let x = 1.0;
    let y = 5.5;
    let octaves = 4;
    let gain = 0.5; // Don't use values greater than 1.0
    println!("{}", noise.perlin_octaves(x, y, octaves, gain)); // Print the noise value at x and y coordinates
}

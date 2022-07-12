use noilib_simple::NoiseGenerator;

fn main() {
    let seed = 123;
    let noise = NoiseGenerator::new(seed); // Initializing noise generator

    let x = 1.0;
    let y = 5.5;
    println!("{}", noise.value(x, y)); // Print the noise value at x and y coordinates
}

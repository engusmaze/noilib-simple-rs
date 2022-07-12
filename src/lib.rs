#[inline(always)]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return (b - a) * t + a;
}
#[inline(always)]
fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    return (b - a) * (t * (t * 6.0 - 15.0) + 10.0) * t * t * t + a;
}

pub struct NoiseGenerator {
    permutations: [i32; 256],
}
impl NoiseGenerator {
    pub fn new(seed: u64) -> Self {
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        let mut seed = seed;
        let mut rand = || {
            seed = A.wrapping_mul(seed).wrapping_add(C);
            seed
        };

        let mut permutations = [0i32; 256];
        for i in 0..256 {
            permutations[i] = rand() as i32;
        }
        Self { permutations }
    }
    fn randomize_2_f(&self, x: i32, y: i32) -> f32 {
        let sum = y.wrapping_mul(506791837).wrapping_add(x);
        // let mut sum2 = 0;
        // sum2 += self.permutations[(sum & 0x000000ff) as usize];
        // for b in sum.to_le_bytes() {
        //     sum2 += self.permutations[b as usize];
        // }
        (self.permutations[(sum & 0x000000ff) as usize]
            .wrapping_add(self.permutations[(sum >> 8 & 0x000000ff) as usize])
            .wrapping_add(self.permutations[(sum >> 16 & 0x000000ff) as usize])
            .wrapping_add(self.permutations[(sum >> 24 & 0x000000ff) as usize])
            % 123456) as f32
            / 123456.0
    }
    fn random_gradient(&self, x: i32, y: i32) -> [f32; 2] {
        let sum = y.wrapping_mul(506791837).wrapping_add(x);
        let dir = (self.permutations[(sum & 0x000000ff) as usize]
            .wrapping_add(self.permutations[(sum >> 8 & 0x000000ff) as usize])
            .wrapping_add(self.permutations[(sum >> 16 & 0x000000ff) as usize])
            .wrapping_add(self.permutations[(sum >> 24 & 0x000000ff) as usize])
            % 123456) as f32
            / 123456.0
            * std::f32::consts::PI
            * 8.0;
        [dir.cos(), dir.sin()]
    }
    fn dot_gradient(&self, xi: i32, yi: i32, x: f32, y: f32) -> f32 {
        let gradient = self.random_gradient(xi, yi);
        let dx = xi as f32 - x;
        let dy = yi as f32 - y;
        dx * gradient[0] + dy * gradient[1]
    }
    pub fn perlin(&self, x: f32, y: f32) -> f32 {
        let xff = x.floor();
        let yff = y.floor();
        let xf = xff as i32;
        let yf = yff as i32;
        let xc = xf + 1;
        let yc = yf + 1;
        let xo = x - xff;
        let yo = y - yff;

        smoothstep(
            smoothstep(
                self.dot_gradient(xf, yf, x, y),
                self.dot_gradient(xc, yf, x, y),
                xo,
            ),
            smoothstep(
                self.dot_gradient(xf, yc, x, y),
                self.dot_gradient(xc, yc, x, y),
                xo,
            ),
            yo,
        ) * std::f32::consts::SQRT_2
    }
    pub fn perlin_octaves(&self, x: f32, y: f32, octaves: usize, gain: f32) -> f32 {
        let mut sum = 0.0;
        let mut scale = 1.0;
        for _ in 0..octaves {
            sum = lerp(sum, self.perlin(x / scale, y / scale), scale);
            scale *= gain;
        }
        sum
    }
    pub fn value(&self, x: f32, y: f32) -> f32 {
        let xff = x.floor();
        let yff = y.floor();
        let xf = xff as i32;
        let yf = yff as i32;
        let xc = xf + 1;
        let yc = yf + 1;
        let xo = x - xff;
        let yo = y - yff;

        return lerp(
            lerp(self.randomize_2_f(xf, yf), self.randomize_2_f(xc, yf), xo),
            lerp(self.randomize_2_f(xf, yc), self.randomize_2_f(xc, yc), xo),
            yo,
        );
    }
    pub fn value_octaves(&self, x: f32, y: f32, octaves: usize, gain: f32) -> f32 {
        let mut sum = 0.0;
        let mut scale = 1.0;
        for _ in 0..octaves {
            sum = lerp(sum, self.value(x / scale, y / scale), scale);
            scale *= gain;
        }
        sum
    }
}

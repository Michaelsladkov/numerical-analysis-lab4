use rand;

pub type Point = (f64, f64);
pub type PointSet = Vec<Point>;

pub enum Clearness {
    CLEAR,
    NOISY
}

pub fn create_set(fun: fn(f64) -> f64, a: f64, b: f64, steps: u32, clr: Clearness) -> PointSet {
    create_set_with_info(fun, a, b, steps, clr).0
}

const NOISE_LEVEL: f64 = 0.01;

pub fn create_set_with_info(fun: fn(f64) -> f64, a: f64, b: f64, steps: u32, clr: Clearness) -> (PointSet, f32, f32) {
    let mut ret: Vec<Point> = Vec::new();
    let mut min = fun(a);
    let mut max = fun(a);
    let step = (b - a) / steps as f64;
    for i in 0..=steps {
        let x = a + i as f64 * step;
        let y = fun(x);
        let noise_rand_source: f64 = rand::random();
        let noise = match clr {
            Clearness::CLEAR => 0.0,
            Clearness::NOISY => (noise_rand_source - 0.5) * 2.0 * NOISE_LEVEL * y,
        };
        ret.push((x, y + noise));
        if y + noise > max {max = y + noise};
        if y + noise < min {min = y + noise};
    }
    (ret, min as f32, max as f32)
}
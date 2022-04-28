use rand;

pub type Point = (f64, f64);
pub type PointSet = Vec<Point>;

pub fn create_set(fun: fn(f64) -> f64, a: f64, b: f64, steps: u32) -> PointSet {
    let mut ret: Vec<Point> = Vec::new();
    let step = (b - a) / steps as f64;
    for i in 0..=steps {
        let x = a + i as f64 * step;
        let y = fun(x);
        let noise_rand_source: f64 = rand::random();
        let noise = (noise_rand_source - 0.5) / 10.0 * y; 
        ret.push((x, y + noise));
    }
    ret
}
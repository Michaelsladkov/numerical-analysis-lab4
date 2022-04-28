use rand;

pub type Point = (f64, f64);
pub type PointSet = Vec<Point>;

pub enum Clearness {
    CLEAR,
    NOISY(f64),
}

pub fn create_set(
    fun: fn(f64) -> f64,
    a: f64,
    b: f64,
    steps: u32,
    clr: Clearness,
) -> Result<PointSet, String> {
    let ret = create_set_with_info(fun, a, b, steps, clr);
    match ret {
        Ok(set_info) => Ok(set_info.0),
        Err(msg) => Err(msg),
    }
}

pub fn create_set_with_info(
    fun: fn(f64) -> f64,
    a: f64,
    b: f64,
    steps: u32,
    clr: Clearness,
) -> Result<(PointSet, f32, f32), String> {
    let mut ret: Vec<Point> = Vec::new();
    let mut min = fun(a);
    let mut max = fun(a);
    let step = (b - a) / steps as f64;
    for i in 0..=steps {
        let x = a + i as f64 * step;
        let y: f64 = fun(x);
        if y.is_nan() {
            return Err(String::from("Function is undefined on this segment"));
        }
        if y.is_infinite() {
            return Err(String::from("Functions has infinite gap on this segment"));
        }
        let noise_rand_source: f64 = rand::random();
        let noise = match clr {
            Clearness::CLEAR => 0.0,
            Clearness::NOISY(level) => (noise_rand_source - 0.5) * 2.0 * level * y,
        };
        ret.push((x, y + noise));
        if y + noise > max {
            max = y + noise
        };
        if y + noise < min {
            min = y + noise
        };
    }
    Ok((ret, min as f32, max as f32))
}

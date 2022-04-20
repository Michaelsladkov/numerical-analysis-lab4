pub type Point = (f64, f64);
pub type PointSet = Vec<Point>;

pub fn create_set(fun: fn(f64) -> f64, a: f64, b: f64, steps: u32) -> PointSet {
    let mut ret: Vec<Point> = Vec::new();
    let step = (b - a) / steps as f64;
    for i in 0..=steps {
        let x = a + i as f64 * step;
        ret.push((x, fun(x)));
    }
    ret
}

pub fn create_polynom(set: PointSet) -> Box<dyn Fn(f64) -> f64> {
    let mut lagrange_denominators: Vec<f64> = Vec::new();
    for i in 0..set.len() {
        let mut d_i = 1f64;
        for j in 0..set.len() {
            if i != j {
                d_i = d_i * (set[i].0 - set[j].0);
            }
        }
        lagrange_denominators.push(d_i);
    }
    return Box::new(move |x: f64| -> f64 {
        let mut ret = 0f64;
        for i in 0..set.len() - 1 {
            let mut l_i = 1f64;
            for j in 0..=set.len() - 1 {
                if i != j {l_i = l_i * (x - set[j].0);} 
            }
            l_i = l_i / lagrange_denominators[i];
            l_i = l_i * set[i].1;
            ret = ret + l_i;
        }
        ret
    });
}
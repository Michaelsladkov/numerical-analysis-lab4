pub mod interpolator;
pub mod points;
pub mod my_io;
pub mod plotter;

use points::Point;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut funcs: Vec<(fn(f64) -> f64, &str)> = Vec::new();
    funcs.push((|x| x * x + 5f64, "y = x^2 + 5"));
    funcs.push((|x| -x * x + 9f64, "y = -x^2 + 9"));
    funcs.push((|_x| 5f64, "y = 5"));
    funcs.push((|x| x + 7f64, "y = x + 7"));
    funcs.push((|x| -0.5 * x + 6f64, "y = -0.5x + 6"));
    funcs.push((
        |x: f64| -> f64 { (2f64 * x).sin() + 3f64 * x },
        "y = sin(2x) + 3x",
    ));
    funcs.push((|x: f64| -> f64 { x.sqrt() }, "y = sqrt(x)"));

    println!("Select one of these functions: ");
    my_io::print_funcs(&funcs, 1);

    let selected_func_index = my_io::get_index(Some(funcs.len())) - 1;
    let selected_func = funcs[selected_func_index];

    let mut test_set: Vec<Point> = Vec::new();
    test_set.push((0f64, 0f64));
    test_set.push((1f64, 1f64));
    test_set.push((2f64, 4f64));
    let test_set = points::create_set(selected_func.0, 0f64, 10f64, 10);
    let test_set_copy = copy_vec(&test_set);
    let y = interpolator::create_polynom(test_set);
    for i in 1..10 {
        println!("{}", y(i as f64));
    }
    plotter::draw_chart(selected_func, y, test_set_copy, 1f32, 10f32);
}

pub fn copy_vec<T: Clone>(vec: &[T]) -> Vec<T> {
    vec.to_vec().to_owned()
}

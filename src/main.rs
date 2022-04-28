pub mod interpolator;
pub mod my_io;
pub mod plotter;
pub mod points;

use points::Clearness::NOISY;
use std::env;

const NOISE_LEVEL: f64 = 0.01;

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
    funcs.push((|x: f64| -> f64 {x.sin()}, "y = sin(x)"));
    funcs.push((|x: f64| -> f64 {x.exp()}, "y = e^x"));

    println!("Select one of these functions: ");
    my_io::print_funcs(&funcs, 1);

    let selected_func_index = my_io::get_index(Some(funcs.len())) - 1;
    let selected_func = funcs[selected_func_index];

    println!("Enter left interpolation border:");
    let a = my_io::get_double();
    println!("Enter right interpolation border:");
    let b = my_io::get_double();
    println!("Enter number of points:");
    let number_of_points = my_io::get_index(None);

    let test_set = points::create_set(
        selected_func.0,
        a,
        b,
        number_of_points as u32,
        NOISY(NOISE_LEVEL),
    );
    let test_set = match test_set {
        Ok(set) => set,
        Err(msg) => {
            println!("Failed to create point set");
            println!("{}", msg);
            return;
        }
    };
    let test_set_copy = copy_vec(&test_set);
    let y = interpolator::create_polynom(test_set);
    match plotter::draw_chart(selected_func, y, test_set_copy, a as f32, b as f32) {
        Ok(_) => (),
        Err(_) => {
            println!("Error while plotting has been occurred");
        }
    }
    println!("Check out.png to see chart");
}

pub fn copy_vec<T: Clone>(vec: &[T]) -> Vec<T> {
    vec.to_vec().to_owned()
}

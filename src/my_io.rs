pub fn get_index(top_limit: Option<usize>) -> usize {
    let line = get_line();
    let mut maybe_index = line.trim().parse::<usize>();
    while maybe_index.is_err() {
        println!("This is not a valid index");
        let line = get_line();
        maybe_index = line.trim().parse::<usize>();
    }
    let index: usize = maybe_index.unwrap();
    if index < 1 {
        println!("Index should be positive");
        return get_index(top_limit);
    }
    if top_limit.is_some() && top_limit.unwrap() < index as usize {
        println!("Index should be not greater than {}", top_limit.unwrap());
        return get_index(top_limit);
    }
    index
}

pub fn get_double() -> f64 {
    let line = get_line();
    let mut maybe_double = line.trim().parse::<f64>();
    while maybe_double.is_err() {
        println!("This is not a valid number");
        let line = get_line();
        maybe_double = line.trim().parse::<f64>();
    }
    maybe_double.unwrap()
}

pub fn get_line() -> String {
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read string");
    input_string
}

pub fn print_funcs(list: &[(fn(f64) -> f64, &str)], num: u16) {
    if list.is_empty() {
        return;
    } else {
        println!("{}. {}", num, list[0].1);
        print_funcs(&list[1..], num + 1);
    }
}

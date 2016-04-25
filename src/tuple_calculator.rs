fn main() {
    println!("1 + 2  = {}", binary_function_calculator(("+", 1, 2)));
    println!("1 - 2  = {}", binary_function_calculator(("-", 1, 2)));
    println!("1 / 2  = {}", binary_function_calculator(("/", 1, 2)));
    println!("1 * 2  = {}", binary_function_calculator(("*", 1, 2)))
}

fn binary_function_calculator(params: (&str, i32, i32)) -> i32 {
    match params {
        ("+", x, y) => x + y,
        ("-", x, y) => x - y,
        ("/", x, y) => x / y,
        ("*", x, y) => x * y,
        _ => std::i32::MIN,
    }
}

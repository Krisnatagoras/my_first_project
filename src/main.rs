// src/main.rs

// Declare the modules. The `mod` keyword tells Rust to look for
// `src/integer_calculations.rs` and `src/float_calculations.rs`.
// These lines make the modules available in the current scope.
mod integer_calculations;
mod float_calculations;
mod string_functions;

fn main() {
    println!("--- Demonstrating Integer Calculations ---");
    let int_a:i32 = 25;
    let int_b:i32 = 7;

    let sum_int = integer_calculations::add(int_a, int_b);
    println!("{} + {} = {}", int_a, int_b, sum_int);

    let diff_int = integer_calculations::subtract(int_a, int_b);
    println!("{} - {} = {}", int_a, int_b, diff_int);

    let prod_int = integer_calculations::multiply(int_a, int_b);
    println!("{} * {} = {}", int_a, int_b, prod_int);

    let div_int = integer_calculations::divide(int_a, int_b);
    println!("{} / {} = {}", int_a, int_b, div_int);

    let div_by_zero_int = integer_calculations::divide(10, 0); // Demonstrates error handling
    println!("10 / 0 = {}", div_by_zero_int);

    // println!("{} + {} = {}", int_a, int_b, integer_calculations::add(int_a, int_b));

    // println!("{} - {} = {}", int_a, int_b, diff_int = integer_calculations::subtract(int_a, int_b));

    // println!("{} * {} = {}", int_a, int_b, prod_int = integer_calculations::multiply(int_a, int_b));

    // println!("{} / {} = {}", int_a, int_b, integer_calculations::divide(int_a, int_b));

    println!("\n--- Demonstrating Float Calculations ---");
    let float_x = 50.51234324155665433542525252353253;
    let float_y = 10.0;

    // let sum_float = float_calculations::add(float_x, float_y);
    // println!("{} + {} = {}", float_x, float_y, sum_float);

    // let diff_float = float_calculations::subtract(float_x, float_y);
    // println!("{} - {} = {}", float_x, float_y, diff_float);

    // let prod_float = float_calculations::multiply(float_x, float_y);
    // println!("{} * {} = {}", float_x, float_y, prod_float);

    // let div_float = float_calculations::divide(float_x, float_y);
    // println!("{} / {} = {}", float_x, float_y, div_float);

    // let div_by_zero_float = float_calculations::divide(10.0, 0.0); // Demonstrates NaN output
    // println!("10.0 / 0.0 = {}", div_by_zero_float);
   
    println!("{} + {} = {}", float_x, float_y, float_calculations::add(float_x, float_y));

    println!("{} - {} = {}", float_x, float_y, float_calculations::subtract(float_x, float_y));

    println!("{} * {} = {}", float_x, float_y, float_calculations::multiply(float_x, float_y));

    println!("{} / {} = {}", float_x, float_y, float_calculations::divide(float_x, float_y));

    let div_by_zero_float = float_calculations::divide(10.0, 0.0); // Demonstrates NaN output
    println!("10.0 / 0.0 = {}", div_by_zero_float);

    println!("\n--- Demonstrating Strings ---");
    let string_x = "Hello from x"; // These are string slices (&str)
    let string_y = "Hello from y";

    println!("{}", string_functions::print_x(string_x));
    println!("{}", string_functions::print_y(string_y));

    // The print_both function will be corrected below.
    // We'll use curly braces {} as placeholders for format!
    println!("{}",  string_functions::print_both(string_x, string_y));

}
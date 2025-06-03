// src/integer_calculations.rs

// Functions in a module need to be public (`pub`) to be used outside the module.
// We'll use i32 for integers, but you can choose other integer types (e.g., i64, u32).

/// Adds two 32-bit integers and returns their sum.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second 32-bit integer from the first and returns the result.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two 32-bit integers and returns their product.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides the first 32-bit integer by the second and returns the result.
/// Handles division by zero by returning 0 and printing an error.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        eprintln!("Error: Division by zero attempted in integer_calculations::divide.");
        0 // Return 0 or consider a Result type for proper error handling
    } else {
        a / b
    }
}

// You can also have private helper functions within the module
// fn is_positive(num: i32) -> bool {
//     num > 0
// }
// src/float_calculations.rs

// Functions in a module need to be public (`pub`) to be used outside the module.
// We'll use f64 for double-precision floating-point numbers.

/// Adds two 64-bit floating-point numbers and returns their sum.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts the second 64-bit floating-point number from the first and returns the result.
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Divides the first 64-bit floating-point number by the second and returns the result.
/// Handles division by zero by returning `f64::NAN` (Not a Number) and printing a warning.
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        eprintln!("Warning: Division by zero attempted in float_calculations::divide.");
        f64::NAN // Not a Number, a standard way to represent undefined float results
    } else {
        a / b
    }
}

/// Multiplies two 64-bit floating-point numbers and returns their product.
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
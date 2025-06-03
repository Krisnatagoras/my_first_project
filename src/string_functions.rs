// src/string_functions.rs

// This function takes a string slice and returns it.
pub fn print_x(x: &str) -> &str {
    x
}

// This function takes a string slice and returns it.
pub fn print_y(y: &str) -> &str {
    y
}

// This function takes two string slices and combines them into a new String.
pub fn print_both(x: &str, y: &str) -> String {
    // Use a format string with placeholders {} for x and y
    format!("{} and {}", x, y)
}
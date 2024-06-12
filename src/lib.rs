// #![crate_name = "rust_syntax"]

// The WBC - Wired Brain Coffee top-level exposed submodules
pub mod coffee;
pub mod tea;

/**
 * Module 3 Exercise File - Rust Syntax Demo
 */

const MY_IMPORTANT_NUMBER: i32 = 42;

/// Returns a number after having calculated the input plus our magic constant
///
/// # Arguments
///
/// * `input` - A 32-bit integer
///
/// # Examples
///
/// ```
/// use wbc::add_constant;
/// let calculation = add_constant(10);
/// assert_eq!(calculation, 52);
/// ```

pub fn add_constant(input: i32) -> i32 {
    input + MY_IMPORTANT_NUMBER
}
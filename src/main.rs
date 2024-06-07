#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;

use module_02::getting_started_with_rust;
use module_03::rust_syntax_and_data_types;
use module_04::ownership_and_borrowing;

fn main() {
    // Module 02 - Getting Started with Rust
    // getting_started_with_rust::demo_configuring_visual_studio_code_for_rust();
    // getting_started_with_rust::demo_anatomy_of_a_rust_program();

    // Module 03 - Rust Syntax and Data Types
    // rust_syntax_and_data_types::demo_rust_syntax();
    // rust_syntax_and_data_types::primitive_data_types_in_rust();
    // rust_syntax_and_data_types::creating_custom_types();
    // rust_syntax_and_data_types::demo_using_data_types();

    // Module 04 - Ownership and Borrowing
    // ownership_and_borrowing::demo_moves_copies_and_clones();
    ownership_and_borrowing::references_and_slices();
}

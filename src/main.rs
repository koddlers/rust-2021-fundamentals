#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;
mod module_05;
mod module_06;
mod module_07;
mod module_08;

use module_02::getting_started_with_rust;
use module_03::rust_syntax_and_data_types;
use module_04::ownership_and_borrowing;
use module_05::control_flow;
use module_06::writing_functions;
use module_07::handling_errors_and_debugging;
use module_08::understanding_basic_collections;

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
    // ownership_and_borrowing::references_and_slices();
    // ownership_and_borrowing::demo_references_and_slices();

    // Module 05 - Control Flow
    // control_flow::control_flow();
    // control_flow::basic_loops();
    // control_flow::while_loops();
    // control_flow::for_loops();
    // control_flow::demo_using_control_flow();
    // control_flow::pattern_matching();
    // control_flow::using_logical_operators();

    // Module 06 - Writing Functions
    // writing_functions::anatomy_of_a_function();
    // writing_functions::using_functions();
    // writing_functions::demo_using_functions();

    // Module 07 - Handling Errors and Debugging
    // handling_errors_and_debugging::handling_errors();
    // handling_errors_and_debugging::demo_error_handling_in_rust();

    // Module 08 - Understanding Basic Collections
    // understanding_basic_collections::demo_using_collections();
    // understanding_basic_collections::iterators();
    understanding_basic_collections::demo_using_iterators();
}

#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;
mod module_05;
mod module_06;
mod module_07;
mod module_08;
mod module_09;
mod module_10;
mod module_11;
mod module_12;

use module_02::getting_started_with_rust;
use module_03::rust_syntax_and_data_types;
use module_04::ownership_and_borrowing;
use module_05::control_flow;
use module_06::writing_functions;
use module_07::handling_errors_and_debugging;
use module_08::understanding_basic_collections;
use module_09::using_traits_and_generics;
use module_09::using_traits_and_generics_v2;
use module_09::using_traits_and_generics_v3;
use module_09::using_traits_and_generics_oop;
use module_10::structuring_rust_code;
use module_10::structuring_rust_code_v2;
use module_11::file_io;
use module_12::concurrency_and_rust;

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
    // understanding_basic_collections::demo_using_iterators();

    // Module 09 - Using Traits and Generics
    // using_traits_and_generics::introduction_to_traits();
    // using_traits_and_generics_v2::demo_using_traits();
    // using_traits_and_generics_v3::using_generics();
    using_traits_and_generics_v3::using_generics_v2();
    // module_09::oop_in_rust();

    // Module 10 - Structuring Rust Code
    // structuring_rust_code::modules();
    // structuring_rust_code_v2::demo_third_party_dependencies();

    // Module 11 - File IO
    // file_io::introduction_to_file_io();
    // file_io::demo_file_io();

    // Module 12 - Concurrency and Rust
    // concurrency_and_rust::using_threads();
    // concurrency_and_rust::message_passing_using_channels();
    // concurrency_and_rust::shared_state_concurrency();
    // concurrency_and_rust::demo_threads_and_channels();
    // concurrency_and_rust::demo_using_the_sync_and_send_traits();
}

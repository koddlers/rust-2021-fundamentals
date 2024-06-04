pub mod getting_started_with_rust {
    // 03 - Demo - Configuring Visual Studio Code for Rust
    pub fn demo_configuring_visual_studio_code_for_rust() {
        let my_string = String::from("Hello, World!");
        println!("{}", my_string);
    }


    use std::io::stdin;

    // 04 - Demo - Anatomy of a Rust Program
    pub fn demo_anatomy_of_a_rust_program() {
        let mut message = String::new();
        println!("Enter your message:");

        stdin().read_line(&mut message).unwrap();
        println!("Message received: {}", message);
    }
}
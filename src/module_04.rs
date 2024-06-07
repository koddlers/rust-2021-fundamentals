pub mod ownership_and_borrowing {
    #[derive(Debug, Clone, Copy)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    pub fn demo_moves_copies_and_clones() {
        // primitive values are stored in the `Stack` memory
        // moves/copies primitives, structs
        let a = 1;
        let b = a;

        // the primitive value type is implicitly copied over here
        println!("a: {}", a);
        println!("b: {}", b);

        // strings are stored in the `Heap`
        let string_a = String::from("Hello");
        let string_b = string_a;

        // uncommenting this code will cause our program to not compile
        // println!("string_a: {}", string_a);
        println!("string_b: {}", string_b);

        let string_c = String::from("World");
        let string_d = string_c.clone();
        println!("string_c: {}", string_c);
        println!("string_d: {}", string_d);

        {
            let greeting = String::from("Hello World");
            println!("Greeting in scope: {}", greeting);
        }

        // uncommenting this line will cause our program to not compile
        // because it is defined in an inner scope above and goes out of
        // scope once the scope is closed by the closing bracket `}`
        // println!("Greeting: {}", greeting);

        let coffee_a = Coffee { id: 1, count: 40 };
        let coffee_b = coffee_a;
        // uncommenting this line will cause our program to not compile
        // ... unless our struct is Copy
        // println!("coffee_a: {:?}", coffee_a);

        let coffee_c = Coffee { id: 1, count: 40 };
        let coffee_d = coffee_c.clone();
        println!("coffee_c: {:?}", coffee_c);
        println!("coffee_d: {:?}", coffee_d);
    }
}
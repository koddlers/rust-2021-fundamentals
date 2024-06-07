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

    fn concat(a: &mut String) {
        a.push_str("bar")
    }

    pub fn references_and_slices() {
        let foo = String::from("foo");
        // the following call to `concat()` sends `foo` as an "Immutable Reference"
        // and WE CANNOT DO THAT
        // concat(&foo);

        let mut other_foo = String::from("foo");
        // the following call to `concat()` sends `other_foo` as a "Mutable Reference"
        // and this is completely fine
        println!("other_foo (before):\t {}", other_foo);
        concat(&mut other_foo);
        println!("other_foo (after):\t {}", other_foo);

        // slices - strings and arrays
        let foobar = String::from("foobar");
        let foo = &foobar[0..4];
        let bar = &foobar[3..];
        println!("foo: {}", foo);
        println!("bar: {}", bar);

        let my_arr = [1, 2, 3, 4];
        let one = &my_arr[..1];
        println!("one: {:?}", one);

        let mut my_mut_arr = [1, 2, 3];
        let my_mut_slice = &mut my_mut_arr[..1];
        my_mut_slice[0] = 50;
        // this following `println!()` will throw an error because we borrowed from `my_mut_arr` already
        // println!("my_mut_arr: {:?}", my_mut_arr);
        println!("my_mut_slice: {:?}", my_mut_slice);
    }
}
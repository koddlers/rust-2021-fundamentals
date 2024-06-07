pub mod control_flow {
    pub fn control_flow() {
        // conditional expressions
        let my_num = 10;

        if my_num >= 5 {
            println!("The number is greater than or equal to 5");
        } else if my_num > 2 {
            println!("The number is greater than 3 but less than 5");
        } else {
            println!("Now we are running this code")
        }

        // conditional expressions with `let` statements
        let should_use_ten = true;
        let my_num = if should_use_ten { 10 } else { 0 };
        println!("my_num: {}", my_num);
    }

    pub fn basic_loops() {
        // basic infinite loops
        // loop {
        //     println!("We are looping!");
        // }

        // returning values using loop
        let mut my_num = 0;
        let final_num = loop {
            my_num += 5;

            if my_num > 10 {
                break my_num + 1;
            }
        };
        println!("final_num: {}", final_num);
    }

    pub fn while_loops() {
        let mut my_num = 0;

        while my_num < 10 {
            my_num += 1;
        }

        println!("my_num: {}", my_num);
    }

    pub fn for_loops() {
        let my_arr = [1, 2, 3, 4, 5];
        for num in my_arr {
            print!("{} ", num);
        }

        // mutable looping
        let mut my_vec = vec![1, 2, 3, 4, 5];

        println!("\nmy_vec (before): {:?}", my_vec);
        for num in &mut my_vec {
            *num += 10;
        }
        println!("my_vec (after): {:?}", my_vec);
    }

    pub fn demo_using_control_flow() {
        let a = 1;
        let b = a + 1;

        println!("Expression evaluation: {}", a + 1 * b);

        // control flow using conditional expressions
        if a == 1 {
            println!("Conditional expression evaluated to true");
        } else {
            println!("Conditional expression evaluated to false");
        }

        let result = if a + 1 == 2 { a } else { b };
        println!("Conditional expression evaluation: {}", result);

        let mut counter = 0;
        loop {
            if counter == 3 { break; }
            counter += 1;
        }

        loop {
            if counter < 10 {
                counter += 1000;
                println!("Continuing to next loop iteration...");
                continue;
            }

            if counter > 1000 {
                println!("Counter is greater than 1000, breaking.");
                break;
            }
        }

        // nested loops with loop labels
        'one: loop {
            'two: loop {
                println!("Breaking out of loop two");
                break 'two;
            }
            println!("In loop one");
            break 'one;
        }

        // assigning values from a loop
        let mut start = 0;
        let result = loop {
            start += 1;
            if start == 20 {
                break start;
            }
        };
        println!("Loop result: {}", result);

        let mut next_counter = 0;
        while next_counter < 10 {
            println!("Counter: {}", next_counter);
            next_counter += 1;
        }

        let my_arr = [1, 2, 3];
        let mut current_index = 0;
        while let Some(number) = my_arr.get(current_index) {
            println!("Index is valid - fetch number: {}", number);
            current_index += 1;
        }

        let my_arr = [1, 2, 3, 4, 5];
        for num in my_arr {
            println!("Number: {}", num);
        }

        let mut my_other_arr = [1, 2, 3];
        for num in &mut my_other_arr {
            *num += 1;
            println!("Mutated number: {}", num);
        }
        println!("Mutated Array: {:?}", my_other_arr);
    }
}
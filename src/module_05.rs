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
}
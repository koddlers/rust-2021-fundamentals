pub mod handling_errors_and_debugging {
    use std::fs::{File, OpenOptions};
    use std::num::ParseIntError;
    use std::{io::Result};

    pub fn handling_errors() {
        let file = File::open("test.txt");

        match file {
            Ok(test_file) => println!("We successfully opened the file"),

            // either print the error or cause a panic, not both
            Err(err) => panic!("Error: {:?}", err)
            // Err(err) => println!("Error: {:?}", err)
        }
    }

    // The following code causes error, especially the `a.parse()?`
    // fn multiply_str_by_ten(a: &str) -> Result<i32, ParseIntError> {
    //     let parsed = a.parse()?;
    //     parsed * 10
    // }
    //
    // pub fn handling_errors_v2() {
    //     let a = multiply_str_by_ten("100");
    //     multiply_str_by_ten("one hundred");
    // }

    #[derive(Debug)]
    struct Coffee {
        id: i32,
        count: i32,
    }

    impl Coffee {
        fn new(id: i32, count: i32) -> Self {
            Self { id, count }
        }

        fn increase_count(&mut self, amount: i32) {
            self.count += amount;
        }

        fn print(&self) {
            println!("Kofi ID: {}, Count: {}", self.id, self.count)
        }
    }


    #[derive(Debug, PartialEq, Clone)]
    struct MyCustomError {
        message: String,
    }

    impl MyCustomError {
        fn new(message: &str) -> MyCustomError {
            MyCustomError { message: message.to_string() }
        }
    }

    fn open_file(path: &str) -> Result<File> {
        OpenOptions::new().read(true).open(path)
    }

    fn open_file_chain(file_one: &str, file_two: &str) -> Result<File> {
        open_file(file_one)?;
        open_file(file_two)
    }

    pub fn demo_error_handling_in_rust() {
        let coffees = Vec::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 2000, count: 20 },
            Coffee { id: 3000, count: 30 },
        ]);

        println!("Vector of coffees: {:?}", coffees);

        // access an invalid index from this vector - handle Option
        let maybe_coffee = coffees.get(4);
        let result = match maybe_coffee {
            None => Err(MyCustomError::new("Coffee did not exist!\n\n")),
            Some(coffee) => Ok(coffee)
        };

        match result {
            Ok(coffee) => println!("{:?}", coffee),
            Err(err) => print!("{}", err.message)
        }

        let open_file_result = open_file("file.txt");
        match open_file_result {
            Ok(file) => println!("{:?}", file),
            Err(err) => println!("this is unexpected")
            // Err(err) => panic!("{:?}", err)
        }

        let open_file_chain_result = open_file_chain("file1.txt", "file2.txt");
        match open_file_chain_result {
            Ok(last_file) => println!("{:?}", last_file),
            Err(err) => println!("{:?}", err)
        }
    }
}
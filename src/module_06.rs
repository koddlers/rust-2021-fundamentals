pub mod writing_functions {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn add_or_subtract(a: i32, b: i32) -> i32 {
        if (a <= b) {
            return a + b;
        }

        a - b
    }

    #[derive(Debug)]
    enum LogLevel {
        INFO,
        WARNING,
        ERROR,
        FATAL,
    }

    // this function returns nothing - it just has side effects
    fn log(level: LogLevel, msg: &str) {
        println!("{:?}: {}", level, msg);
    }

    pub fn anatomy_of_a_function() {
        let (a, b) = (1, 2);
        let mut result = add(a, b);
        println!("add({}, {}) -> {}", a, b, result);

        result = add_or_subtract(b, a);
        println!("add_or_subtract({}, {}) -> {}", b, a, result);

        log(LogLevel::INFO, "The application is running fine at the moment");
    }

    // Methods - example
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn print(&self) {
            println!("x: {} | y: {}", self.x, self.y);
        }

        fn add_one(&mut self) {
            self.x += 1;
            self.y += 1;
        }
    }

    // Associated Functions - Example
    struct Coffee {
        temp: i32,
        name: String,
    }

    impl Coffee {
        fn new(temp: i32, name: &str) -> Coffee {
            Coffee {
                temp,
                name: name.to_string(), // Convert &str to String
            }
        }
    }

    pub fn using_functions() {
        /// There are four types of functions in Rust
        // 1. Named Functions
        fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
        let a = 20;
        let b = 10;
        println!("Named Function, `subtract({}, {})` -> {}", a, b, subtract(a, b));

        // 2. Anonymous Functions or Closures
        let anon = |a: i32, b: i32| if a > b { a + b } else { a - b };
        println!("Anonymous Function, `anon({}, {})` -> {}", a, b, anon(a, b));

        let my_num = 42;
        // this function has access to a variable defined outside of its own scope
        // i.e. it can capture variable defined above its scope
        let my_anon_fn = |a: i32| if a < 0 { a + my_num } else { a - my_num };
        println!("Anonymous Function Capturing Variables in Enclosing Scope");
        println!("\t`my_anon_fn({})`, `my_num = {}` -> {}", a, my_num, my_anon_fn(a));

        // 3. Methods
        let mut point = Point { x: 5, y: 10 };
        point.print();
        point.add_one();
        point.print();


        // 4. Associated Functions
        let coffee = Coffee::new(
            70,
            "Coffea arabica",
        );
        println!("Coffee object created by Associated Function");
        println!("Coffee name: {} and Temperature: {}", coffee.name, coffee.temp);
    }
}
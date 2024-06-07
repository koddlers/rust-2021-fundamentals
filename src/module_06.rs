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
}
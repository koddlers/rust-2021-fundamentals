pub mod using_traits_and_generics {
    // Any type that implements this trait can calculate a total of some kind
    pub trait Total {
        fn calc_total(&self) -> i64;
    }

    // Any type that implements this trait can generate/display a description
    trait Describe {
        fn description(&self) -> String;
    }

    struct Snack {
        name: String,
        item_number: i32,
        cost: f64,
    }

    struct Coffee {
        name: String,
        cost: f64,
        size: String,
    }

    impl Describe for Snack {
        fn description(&self) -> String {
            format!("{} / Item #: {} / Cost: {}", self.name, self.item_number, self.cost)
        }
    }

    impl Describe for Coffee {
        fn description(&self) -> String {
            format!("{} {} costing {}", self.size, self.name, self.cost)
        }
    }

    pub fn introduction_to_traits() {
        let snack = Snack {
            name: String::from("Potato Chips"),
            cost: 10.0,
            item_number: 1,
        };
        println!("{}", snack.description());

        let coffee = Coffee {
            name: String::from("Coffee Arabica"),
            cost: 1000.0,
            size: String::from("200 gm"),
        };
        println!("{}", coffee.description())
    }
}
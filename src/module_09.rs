use crate::module_09::using_traits_and_generics_oop::{Brew, Roast, TempCategory};

pub mod using_traits_and_generics {
    use std::collections::BTreeSet;

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
        println!("{}", coffee.description());
    }
}

pub mod using_traits_and_generics_v2 {
    pub trait Brew {
        fn brew(&self) -> () {
            println!("Brewing...");
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum TempCategory {
        HOT,
        ICED,
    }

    #[derive(Debug, Clone, Copy)]
    enum Roast {
        DARK,
        MEDIUM,
        LIGHT,
    }

    #[derive(Debug, Clone)]
    struct Coffee {
        name: String,
        temp: TempCategory,
        roast: Roast,
    }

    impl Brew for Coffee {
        fn brew(&self) -> () {
            println!("Brewing {:?}, {:?} roast coffee named {}!",
                     self.temp, self.roast, self.name);
        }
    }

    struct Espresso {
        temp: TempCategory,
        brand: String,
    }

    impl Brew for Espresso {
        fn brew(&self) -> () {
            println!("Brewing {}, {:?} espresso!", self.brand, self.temp);
        }
    }

    struct Tea {
        temp: TempCategory,
        origin: String,
        brand: String,
        rating: i32,
    }

    impl Brew for Tea {
        fn brew(&self) -> () {
            println!("Brewing {}, {:?} tea from {} with rating {}",
                     self.brand, self.temp, self.origin, self.rating);
        }
    }

    struct Beer {}

    impl Brew for Beer {}


    pub fn demo_using_traits() {
        let coffee = Coffee {
            name: String::from("Verano"),
            temp: TempCategory::HOT,
            roast: Roast::DARK,
        };
        // coffee.brew();

        let espresso = Espresso {
            temp: TempCategory::HOT,
            brand: String::from("Nespresso"),
        };
        // espresso.brew();

        let tea = Tea {
            origin: String::from("Bangladesh"),
            temp: TempCategory::HOT,
            brand: String::from("Mysterious Tea Company"),
            rating: 100,
        };
        // tea.brew();

        let beer = Beer {};
        // beer.brew();

        // This function takes any reference to an item that implements the Brew trait
        // This is an example of polymorphism in Rust - an object-oriented programming feature
        fn brew_drink(drink: &impl Brew) {
            drink.brew()
        }

        brew_drink(&coffee);
        brew_drink(&espresso);
        brew_drink(&tea);
        brew_drink(&beer);

        // the following code is courtesy of MS Copilot
        // Create a Vec to hold instances of different types
        // let drinks: Vec<Box<dyn Brew>> = vec![
        //     Box::new(coffee),
        //     Box::new(espresso),
        //     Box::new(tea),
        //     Box::new(beer),
        // ];

        // for drink in drinks {
        //     drink.brew();
        // }
    }
}

pub mod using_traits_and_generics_v3 {
    #[derive(Debug)]
    struct Coffee<'a, T, U> {
        name: &'a str,
        cost: T,
        size: U,
    }

    pub fn using_generics() {
        let my_coffee = Coffee {
            name: "Drip",
            cost: 4.99,
            size: 3,
        };
        println!("my_coffee: {:?}", my_coffee);

        let my_other_coffee = Coffee {
            name: "Drip",
            cost: 3.50,
            size: "Grande",
        };
        println!("my_other_coffee: {:?}", my_other_coffee);
    }

    // pub trait Rate {
    //     fn calc_rating(&self) -> i32;
    // }
    //
    // // TODO: fix this function
    // fn best_rating<'a, T: Rate>(items: Vec<T>) -> &'a T {
    //     let mut best= &items[0];
    //
    //     for item in items.iter() {
    //         if item.calc_rating() > best.calc_rating() {
    //             best = item;
    //         }
    //     }
    //
    //     best
    // }
    //
    // #[derive(Debug)]
    // struct Item {
    //     name: String,
    //     rating: i32,
    // }
    //
    // impl Rate for Item {
    //     fn calc_rating(&self) -> i32 {
    //         return self.rating / 10 * 100;
    //     }
    // }
    //
    // pub fn using_generics_v2() {
    //     let candy = Item {
    //         name: String::from("Candy"),
    //         rating: 7,
    //     };
    //
    //     let chocolate = Item {
    //         name: String::from("Chocolate"),
    //         rating: 10,
    //     };
    //
    //     let items = vec![candy, chocolate];
    //     let best_item = best_rating(items);
    //     println!("The best Item is: {:?}", best_item);
    // }
}

pub mod using_traits_and_generics_oop {
    #[derive(Debug)]
    pub enum TempCategory {
        HOT,
        ICED,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Roast {
        DARK,
        MEDIUM,
        LIGHT,
    }

    #[derive(Debug)]
    pub struct Coffee {
        pub name: String,
        pub temp: TempCategory,
        pub roast: Roast,
        cost: f64,
        count: i32,
        total: f64,
    }

    // Encapsulation of the calculation of total and getting/setting of other fields
    impl Coffee {
        // This is akin to a public constructor - notice how we calculate total behind the scenes
        pub fn new(name: String, temp: TempCategory, roast: Roast, cost: f64, count: i32) -> Coffee {
            Coffee {
                name,
                temp,
                roast,
                cost,
                count,
                total: cost * count as f64,
            }
        }

        pub fn get_total(&self) -> f64 {
            self.total
        }

        pub fn set_cost(&mut self, cost: f64) {
            self.cost = cost;
            self.total = self.cost * self.count as f64;
        }

        pub fn set_count(&mut self, count: i32) {
            self.count = count;
            self.total = self.cost * self.count as f64;
        }
    }

    pub trait Drink {
        fn drink(&self) -> () {
            println!("Drinking...");    // default implementation
        }
    }

    pub trait Brew: Drink {
        fn brew(&self) -> () {
            println!("Brewing...")
        }
    }

    // `Drink` is the `super` trait of `Brew`, i.e. the `parent`
    impl Drink for Coffee {
        fn drink(&self) -> () {
            println!("Drinking {:?}, {:?} roast coffee named {}", self.temp, self.roast, self.name);
        }
    }

    // `Brew` is the `child` trait of `Drink`
    impl Brew for Coffee {
        fn brew(&self) -> () {
            println!("Brewing {:?}, {:?} roast coffee named {}", self.temp, self.roast, self.name);
        }
    }
}

use using_traits_and_generics_oop::*;

pub fn oop_in_rust() {
    let mut coffee = Coffee::new(
        String::from("Verano"),
        TempCategory::HOT,
        Roast::DARK,
        3.50,
        50,
    );

    // This code won't compile! The cost field is private!
    // coffee.cost = coffee.cost + 10;

    coffee.set_cost(4.50);
    println!("New total: {}", coffee.get_total());

    coffee.set_count(500);
    println!("New total: {}", coffee.get_total());

    // Pseudo-inheritance via traits test
    fn brew_then_drink(drink: &impl Brew) {
        drink.brew();
        drink.drink();
    }

    brew_then_drink(&coffee);
}
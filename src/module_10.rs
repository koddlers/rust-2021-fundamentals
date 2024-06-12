pub mod structuring_rust_code {
    mod snack {
        #[derive(Debug)]
        pub struct Snack {
            pub name: String,
            pub rating: i32,
            pub cost: f64,
        }
    }


    // Use private inline module
    use snack::Snack;

    use wbc::coffee::Coffee;
    use wbc::coffee::espresso::Espresso;    // Nested module
    use wbc::tea::*;

    pub fn modules() {
        let test_snack = Snack {
            name: String::from("Popcorn"),
            rating: 99,
            cost: 2.49,
        };
        println!("Snack: {:?}", test_snack);

        let test_coffee = Coffee {
            name: String::from("Drip Coffee"),
            cost: 2.55,
            count: 10,
        };
        println!("Coffee: {:?}", test_coffee);

        let test_espresso = Espresso {
            brand: String::from("Nespresso"),
            cost: 3.48,
        };
        println!("Espresso: {:?}", test_espresso);

        let test_tea = Tea {
            brand: String::from("Harley & Sons"),
            cost: 1.48,
        };
        println!("Tea: {:?}", test_tea);
    }
}

pub mod structuring_rust_code_v2 {
    use rusqlite::{Connection, Result, Error, params};

    struct Coffee {
        name: String,
        description: String,
        count: i32,
    }

    pub fn demo_third_party_dependencies() -> Result<(), Error> {
        // Create our database if it doesn't exist
        let conn = Connection::open("coffee.db")?;
        let sql_command = "CREATE TABLE IF NOT EXISTS coffee (
                id integer primary key,
                name text not null unique,
                description text,
                count integer
            )";

        match conn.execute(sql_command, []) {
            Ok(updated) => println!("Update: {:?}", updated),
            Err(err) => println!("Update failed: {}", err)
        };

        let coffee = Coffee {
            name: String::from("Drip"),
            description: String::from("Nice, dark roast"),
            count: 32,
        };

        match conn.execute(
            "INSERT INTO coffee (name, description, count) VALUES (?1, ?2, ?3)",
            [coffee.name, coffee.description, coffee.count.to_string()],
        ) {
            Ok(updated) => println!("Coffee inserted. Update: {}", updated),
            Err(err) => println!("Insertion failed: {}", err)
        };

        Ok(())
    }
}
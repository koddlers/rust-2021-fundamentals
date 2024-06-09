pub mod understanding_basic_collections {
    use std::collections::HashMap;

    pub fn demo_using_collections() {
        // Creating vectors
        // i32 vector
        let mut int_vector: Vec<i32> = Vec::new();
        for i in 1..=10 {
            int_vector.push(i);
        }
        println!("int_vector: {:?}", int_vector);

        let mut prime_nums = vec![2, 3, 5, 7, 11];
        println!("prime_nums: {:?}", prime_nums);

        let mut even_nums = Vec::from([2, 4, 6, 8, 10]);
        println!("even_nums: {:?}", even_nums);

        let maybe_removed = even_nums.pop();
        println!("maybe_removed: {:?}", maybe_removed.unwrap());
        println!("even_nums: {:?}", even_nums);

        let maybe_num = even_nums.get(0);
        match maybe_num {
            Some(num) => println!("Retrieved number: {}", num),
            None => println!("No number at this index")
        }

        match maybe_removed {
            Some(removed) => println!("Removed value: {}", removed),
            None => println!("No number removed")
        }

        // Creating HashMaps and basic operations
        let mut my_map = HashMap::new();
        my_map.insert(1, "Hello");
        my_map.insert(2, "World");
        println!("my_map: {:?}", my_map);

        let key = 8;
        let maybe_removed_val = my_map.remove(&key);
        match maybe_removed_val {
            Some(removed) => println!("Removed entry with value: {}", removed),
            None => println!("Key {} doesn't exist", key)
        }

        match my_map.get(&1) {
            Some(str_slice_ref) => println!("Value: {}", *str_slice_ref),
            None => println!("Entry did not exist")
        }

        let my_coffee_map = HashMap::from([
            ("Drip", 2.99),
            ("Espresso", 4.50)
        ]);
        println!("My Coffee Map: {:?}", my_coffee_map);

        let capacity_map: HashMap<i32, &str> = HashMap::with_capacity(10);
        println!("My coffee map with capacity: {:?}\n", capacity_map.capacity());
    }

    pub fn iterators() {
        // implicit iterators
        let my_nums = vec![1, 2, 3];
        for num in my_nums {
            print!("{} ", num)
        }
        println!();

        // behind the scenes, the code above is using iterators implicitly
        // which will look like the following, explicitly
        let my_nums = vec![1, 2, 3];
        for num in my_nums.iter() {
            print!("{} ", num)
        }
        println!();
    }

    #[derive(Debug, Clone)]
    struct Coffee {
        id: i64,
        count: i64,
    }

    pub fn demo_using_iterators() {
        let coffees = Vec::from([
            Coffee { id: 1000, count: 10 },
            Coffee { id: 2000, count: 20 },
            Coffee { id: 3000, count: 30 },
        ]);

        let coffee_iter = coffees.iter();
        println!("Vector iterator: {:?}", coffee_iter);

        // Iterator adapters - some consume and some don't
        let total: i64 = coffee_iter.map(|coffee| coffee.count).sum();
        println!("Total count: {}", total);

        let coffee_map = HashMap::from([
            ("Coffee 1", Coffee { id: 1000, count: 10 }),
            ("Coffee 2", Coffee { id: 2000, count: 40 }),
            ("Coffee 3", Coffee { id: 3000, count: 500 }),
        ]);
        let coffee_map_iter = coffee_map.iter();
        println!("Coffee map iterator: {:?}", coffee_map_iter);

        let filtered_coffees: Vec<Coffee> = coffee_map_iter
            .filter(|(_key, value)| value.count >= 40)
            .map(|(_ket, value)| value)
            .cloned()
            .collect();

        for coffee in filtered_coffees {
            println!("Coffee: {:?}", coffee);
        }

        // This code won't compile because 'filter' takes ownership of this iterator
        // for (key, value) in coffee_map_iter {
        //     println!("Original Coffee Entry: ({} | {:?})", key, value);
        // }

        // This is just fine though!
        for (key, value) in coffee_map.iter() {
            println!("Original Coffee Entry: ({} | {:?})", key, value);
        }
    }
}
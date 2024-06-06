pub mod rust_syntax_and_data_types {
    use std::io::stdin;
    use rust_syntax::add_constant;

    pub fn demo_rust_syntax() {
        let mut number_str = String::new();
        println!("Enter your number:");

        stdin().read_line(&mut number_str).unwrap();

        // Parse our string as number after trimming off the extra characters
        let number: i32 = number_str.trim().parse().unwrap();
        println!("Calculation: {}", add_constant(number));
    }

    pub fn primitive_data_types_in_rust() {
        // integer types
        let my_number: i32 = 1;
        println!("{}", my_number);

        let my_8_bit_int: i8 = -128;
        println!("{}", my_8_bit_int);

        let other_int: u8 = 128;
        println!("{}", other_int);

        println!(
            "Integers have the following different sizes:
            8-bit\t i8\t u8
            16-bit\t i16\t u16
            32-bit\t i32\t u32
            64-bit\t i64\t u64
            128-bit\t i128\t u128
            arch\t isize\t usize
            (isize and usize depends on CPU architecture)"
        );

        // floating point types are either 32-bit or 64-bit
        let my_floating_point: f64 = 1.234567891011;
        println!("f64: {}", my_floating_point);

        let other_float: f32 = 2.010245;
        println!("f32: {}", other_float);

        // booleans are either true or false
        let this_is_true: bool = true;
        println!("bool: {}", this_is_true);

        let this_is_false: bool = false;
        println!("bool: {}", this_is_false);

        // characters are of unicode type and thus can print this emoji
        let my_char: char = '😐';
        println!("char: {}", my_char);

        // primitive compound types
        // tuples can hold data of multiple scalar types
        let my_tuple = ('A', 1, 1.2);
        println!("tuple: {:?}", my_tuple);

        // arrays on the other hand can hold the data of only one scalar type,
        // i.e. they are heterogeneous
        let my_arr = [1, 2, 3];
        println!("array: {:?}", my_arr);

        // the type of data an array holds can be declared during its definition
        // in which case the size must also be declared
        let my_int32_array: [i32; 3] = [1, 2, 3];
        println!("i32 array: {:?}", my_int32_array);
    }

    #[derive(Debug)]
    struct Coffee {
        id: i64,
        name: String,
        coffee_type: CoffeeType,
    }

    // #[derive(Debug)]
    // enum CoffeeType {
    //     HOT(f32),
    //     ICED(f32),
    // }
    #[derive(Debug)]
    enum CoffeeType {
        HOT(Option<f64>),
        ICED(Option<f64>),
    }

    pub fn creating_custom_types() {
        let coffee = Coffee {
            id: 123456,
            name: String::from("Latte"),
            coffee_type: CoffeeType::HOT(Some(102.5)),
        };

        println!("Coffee ID: {}, Coffee Name: {}, Coffee Type: {:?}",
                 coffee.id, coffee.name, coffee.coffee_type);
    }

    pub fn demo_using_data_types() {
        // integers
        let my_num: i32 = 32;
        println!("Integer: {}", my_num);

        // converting strings to integers
        let parsed_num: i32 = "123".parse().unwrap();
        println!("Parsed num: {}", parsed_num);
        println!("Integer to string: {}", parsed_num.to_string());

        // floats
        let my_float: f32 = 10.5;
        println!("Floor: {}", my_float.floor());
        println!("Ceiling: {}", my_float.ceil());
        println!("Rounded: {}", my_float.round());

        let my_int = my_float as i32 + 1;
        let my_new_float = 1 as f32 + my_float;     // equivalent to `1f32 + my_float`
        println!("Coercing float to int: {}", my_int);
        println!("Coercing int to float: {}", my_new_float);

        // characters
        let my_char = 'A';
        println!("Is uppercase: {}", my_char.is_uppercase());
        println!("Is lowercase: {}", my_char.is_lowercase());
        println!("Lowercase: {}", my_char.to_ascii_lowercase());
        println!("String version: {}", my_char.to_string());

        // booleans
        let my_bool = true;
        assert_eq!(my_bool, true);

        // tuples
        let my_tuple = ('A', 5, 10.5);
        println!("Char/Integer/Float: {}/{}/{}", my_tuple.0, my_tuple.1, my_tuple.2);

        // destructuring tuple values into variables
        let (letter, integer, float_num) = my_tuple;
        println!("Char/Integer/Float: {}/{}/{}", letter, integer, float_num);

        let nested_tuple = ((1, 2), (3, 4));
        let ((a, b), (c, d)) = nested_tuple;
        println!("(({}, {}), ({}, {}))", a, b, c, d);

        // arrays
        let my_arr = [1, 2, 3, 4, 5];
        for num in my_arr {
            println!("Number: {}", num);
        }

        let same_vale_arr: [i32; 1000] = [10; 1000];
        println!("Array: {:?}", same_vale_arr);
        println!("First Element: {}", same_vale_arr[0]);
        println!("Array Length: {}", same_vale_arr.len());
        println!("Array Size: {}", std::mem::size_of_val(&same_vale_arr));

        // structs
        let mut my_coffee = Coffee {
            id: 10,
            name: String::from("Riley"),
            coffee_type: CoffeeType::HOT(None),
        };

        my_coffee.id = 1000;

        let id: i64 = 10;
        let coffee_with_temp = Coffee {
            id,
            name: String::from("Riley"),
            coffee_type: CoffeeType::HOT(Some(103.2)),
        };

        // combined structs
        let combined = Coffee {
            id: 2000,
            ..coffee_with_temp
        };

        println!("Combined: {:?}", combined);
    }
}
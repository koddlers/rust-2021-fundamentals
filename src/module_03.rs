pub mod rust_syntax_and_data_types {
    use std::io::stdin;
    use rust_syntax::add_constant;

    pub fn demo_rust_syntax() {
        let mut number_str = String::new();
        println!("Enter your number:");

        stdin().read_line(&mut number_str).unwrap();02

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
}
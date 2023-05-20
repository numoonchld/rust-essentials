pub fn primitive_data_types() {
    /*  IMMUTABLE, TYPE-INFERRED */
    let x = 10;

    // print new-lin macro
    println!("x is {}", x);

    /* MUTABILITY */
    let mut y = 10;
    println!("y is: {}", y);
    y = 20;
    println!("y is now: {}", y);
}

pub fn integers() {
    let mut x = -10; // default type: i32
    println!("x is {}", x);
    x = 10;
    println!("x is {}", x);
}

pub fn floating_points() {
    let x = 10.123456789123456789;
    println!("x (f64) is {}", x);

    let y: f32 = 10.123456789123456789;
    println!("y (f32) is {}", y)
}

pub fn arithmetic_operations() {
    let a = 10;
    let b = 3;
    println!("a is {} and b is {}", a, b);
    println!("a + b is {}", a + b);
    println!("a - b is {}", a - b);
    println!("a * b is {}", a * b);
    println!("a / b is {}", a / b);
    println!("a % b is {}", a % b);

    let c = 10.0;
    let d = 3.0;
    println!("c is {} and d is {}", a, d);
    println!("c / d is {}", c / d);

    println!("a / d is {} ", a as f64 / d);
}

pub fn bitwise_operations() {
    let value = 0b1111_0101u8;
    println!("value is {value}");
    println!("value is {:08b}", value);

    // BITWISE NOT
    println!("!value is {:08b}", !value);

    // BITWISE AND
    let not_value_and = !value & 0b1111_0111u8;
    println!("!value & 0b1111_0111 is {:08b}", not_value_and);
    // read value with AND
    println!("bit 6 is {:b}", not_value_and & 0b0100_0000);
    println!("bit 1 is {:b}", not_value_and & 0b0000_0010);

    // BITWISE OR
    // set value with OR
    println!(
        "!value | 0b0100_0000 is {:08b}",
        not_value_and | 0b0100_0000
    );

    // BITWISE XOR
    println!(
        "!value ^ 0b0101_0101 is {:08b}",
        not_value_and ^ 0b0101_0101
    );

    // BIT SHIFT
    // left shift
    println!("value left shift 4 is {:08b}", value << 4);
    // right shift
    println!("value right shift 2 is {:08b}", value >> 2);
}

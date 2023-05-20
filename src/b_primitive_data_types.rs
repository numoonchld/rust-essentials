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

pub fn formatting_print() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;

    println!("c is {}", c);
    println!("c is {:.3}", c);
    println!("c is {:8.3}", c);
    println!("c is {:08.3}", c);
    println!("c is {:08.3}\na is {}", c, a);

    print!("hello!"); // prints no new line at the end of the line

    println!("c is {0:08.3}, a is {1}, once again, c is {0}", c, a);
}

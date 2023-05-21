pub fn main() {
    say_a_number(54);
    say_the_sum(4, 5);

    let result = square(5);
    println!("square of 5 is {result}");

    let (number, cube) = cube(14);
    println!("cube of {number} is {cube}");
}

fn say_a_number(number: i32) {
    println!("number is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    println!("the sum is {}", a + b);
}

fn square(x: i32) -> i32 {
    x * x // same as =>  return x * x;
}

fn cube(x: i32) -> (i32, i32) {
    return (x, x * x * x);
}

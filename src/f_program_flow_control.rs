pub fn conditionals() {
    let x = 3;

    if x == 3 {
        println!("x is 3!");
    }

    if x > 0 {
        println!("x greater than 0");
    }

    if x + 1 != 3 {
        println!("x + 1 is not equal to 3!");
    }

    let x = 5;
    let y = 3;

    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is not greater than y");
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x and y are equal!")
    }
}

pub fn conditional_assignment() {
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };

    // if make_x_odd {
    //     x = 1;
    // } else {
    //     x = 2;
    // }

    println!("x is {x}")
}

pub fn loops() {
    let mut count = 0;

    loop {
        count += 1;
        println!("count is {count}");
        if count == 10 {
            break;
        }
    }

    println!("after the loop!");

    let result = loop {
        if count == 20 {
            break count * 10;
        }
        count += 1;
        println!("count is {count}");
    };

    println!("result is {result}");
}

pub fn while_loops() {}

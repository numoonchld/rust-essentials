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

pub fn while_loops() {
    let mut while_count = 0;

    while while_count < 10 {
        while_count += 1;
        println!("while_count is {while_count}");
    }

    // iterating over an array:
    let letters = ['a', 'b', 'c'];
    let mut while_count_2 = 0;

    while while_count_2 < letters.len() {
        println!("letter is {}", letters[while_count_2]);
        while_count_2 += 1;
    }
}

pub fn for_loops() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("item is {item}");
    }

    for (index, item) in message.iter().enumerate() {
        println!("item {index} is {item}");
    }

    for (index, &item) in message.iter().enumerate() {
        println!("item {index} is {item}");
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {number}");
    }
}

pub fn nested_loops() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix {
        for col in row {
            print!("{col}\t");
        }
        println!();
    }

    let mut matrix_2 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix_2.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{num}\t")
        }
        println!()
    }
}

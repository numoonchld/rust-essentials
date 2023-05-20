pub fn arrays() {
    let letters = ['a', 'b', 'c'];
    let first_letter = letters[0];

    println!("first_letter is {first_letter}");

    let mut letters_b = ['a', 'b', 'c'];

    letters_b[0] = 'x';

    let first_letter = letters_b[0];

    println!("first_letter is {first_letter}");

    let numbers: [i32; 5];
    numbers = [0; 5];
    println!("numbers is {:?}", numbers);
    println!("last number is {}", numbers[4])
}

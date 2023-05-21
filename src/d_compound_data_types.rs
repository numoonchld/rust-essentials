pub fn arrays() {
    let letters = ['a', 'b', 'c'];
    println!("letters is {:?}", letters);
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

pub fn multi_dimensional_arrays() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    println!("parking lot is {:?}", parking_lot);
    let number = parking_lot[0][1];
    println!("number is {}", number);

    // initialize a 3D array:
    let garage = [[[0; 10]; 5]; 3];
    println!("garage array is: {:?}", garage)
}

pub fn tuples() {
    let tuple_stuff = (10, 3.14, 'x');
    println!("tuple is {:?}", tuple_stuff);
    println!("first item is {}", tuple_stuff.0);

    let mut tuple_stuff_2: (u8, f32, char) = (10, 3.14, 'y');
    tuple_stuff_2.0 += 3;
    println!("first value increased by 3 is: {:?}", tuple_stuff_2.0);

    // destructuring tuples
    let (_a, b, _c) = tuple_stuff;
    println!("b is {b}")
}

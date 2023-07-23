use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    // let mut deleted: bool = false;
    // let mut decimal: f32 = 3.24;
    // let z: char = 'Z';
    // let heart_eyed_cat: char = '😻';

    // // Tuples are fixed in length
    // let tup: (i32, f64, u8) = (500, 6.4, 2);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let five_hundred = tup.0;
    // let six_point_four = tup.1;
    // let two = tup.2;

    // empty tuple () is named unit -> default return value for expressions if no other value is returned
    // Arrays have a fixed length of same type
    let a: [i32; 7] = [1, 2, 3, 4, 5, 6, 7]; // arrays are allocated on the stack
    let _b: [i32; 5] = [4; 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // usize is the size of your computer's architecture (i.e., 32-bit, 64-bit)
    // thus, it is useful for indexing (so you get the right position)
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

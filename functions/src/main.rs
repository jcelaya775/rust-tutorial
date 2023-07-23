// must specify return type
fn five() -> i32 {
    5 // implicit return
}

fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // Note: Statements are instructions that do not return a value
    // Expressions do return a value

    // scope blocks are expressions
    let y = {
        let x = 3;
        x + 1 // if you include a semicolon, you turn this into a statement (will not return value)
    };
    println!("{y}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The value of x is: {value}{unit_label}");
}

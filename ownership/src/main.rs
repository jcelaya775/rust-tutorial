fn main() {
    let _literal = "hliiii";

    let mut s1: String = String::from("hello");
    let s2 = s1; // s1 moves to s2 (not shallow copy b/c s1 now invalid)
    let mut s3: String = s2.clone(); // deep copy of stack and heap data
    s3.push_str(" string");

    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // stack memory example
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // Note: x is still valid b/c an actual copy is made for y
                                      // hence, x is still valid since double free error doesn't exist
                                      // this is a equivalent to making a deep copy w/ clone()

    // There is a Copy trait that treats variables given this trait the same as simple scalar types
    // i.e., variables do not move and are copied, keeping them valid.
    // This is not available for types that implement the Drop trait
    // Any group of simple scalar values can implement copy (anything that requires allocation cannot)
    let mut s2 = String::from("sup doc");
    let s3 = takes_and_give_back(s2); // s2 moves into takes_and_give_back() which moves it into s3

    let str: String = String::from("wazza");
    let len = calculate_length(&str);
    println!("The length of {str} is {len}.");
    // NOTE: & denotes reference, they allow you to refer to some value
    // w/out taking ownership of it

    let mut myStr: String = String::from("hello");
    change(&mut myStr);

    let r1 = &mut myStr;
    // let r2 = &mut myStr; // can't borrow s as mutable more than once at a time
    // Prevents data races (2+ ptrs where at least 1 writes to data w/out sync)

    // We can still allow for multiple mutable references, just not simulatenously:
    {
        let r2 = &mut myStr;
        println!("{}", r2);
    }

    println!("{}", myStr);
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 // b/c r1 & r2 don't expect value to change

    // println!("{}, {}, and {}", r1, r2, r3);

    // Note: a reference's scope ends where it is last used

    // Dangling references
    let reference_to_nothing = dangle();
}

fn takes_and_give_back(string: String) -> String {
    string
}

// You can return multiple values w/ a tuple
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// Using a reference parameter instead
// References are guaranteed to point to a valid value for the life of that
// reference unlike pointers. Immutable by default
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But b/c it doesn't have ownership of
  // what it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", bob");
}

// fn dangle() -> &String {
//     // dangle returns a ref to a String
//     let s = String::from("hello"); // s is a new String

//     &s // return ref to String s
// } // Here, s goes out of scope, and is dropped.
//   // Danger!

// Fix by moving ownership instead of borrowing reference
fn dangle() -> String {
    let s = String::from("hello");

    s
}

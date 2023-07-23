fn main() {
    let s = String::from("hello there world");
    let word = first_word(&s[..]);
    let word = first_word(&s); // also works b/c equiv to whole slice of s
    println!("{}", word);

    let literal = "hello, world!"; // type of literal is &str
                                   // it's a slice pointing to a point in binary (immutable)

    // W/ arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// Note: &str is the type of a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    // equivalent to:
    let s = String::from("initial contents");

    // UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str() takes a string slice because we don’t want to take ownership of the param
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push() takes a single character as a param and adds it to the String
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); // lol

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used (s1 is not valid, but s2 is)
                       // reason is that add function (+) signature is fn add(self, s: &str) -> String

    // Concatenation multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // this is not very readable and is error prone

    // Using format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; // error: Rust strings don’t support indexing
    // Rust strings don’t support indexing because indexing operations are expected to always take constant time (O(1)).
    // Strings can be viewed as bytes, scalar values, and grapheme clusters (letters).

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s will be a &str that contains the first 4 bytes of the string
                          // If you attempt to create a substring from an invalid index, your program will exit with a panic.
}

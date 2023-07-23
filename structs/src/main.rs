use std::{cmp, println};

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    // Create an instance of the User struct
    let mut user1 = User {
        active: true,
        username: String::from("jcelaya775"),
        email: String::from("jcelaya775@gmail.com"),
        sign_in_count: 3,
    };
    user1.email = String::from("fakebob@bob.com");

    println!(
        "active: {}, username: {}, email: {}, sign_in_count: {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    println!("{}", min(623, 234));
    println!("{}", cmp::min(23423142, 234));

    // let user2: User = build_user(&String::from("jcel"), &String::from("jcelay@mgi.com"));
    let user2: User = build_user(String::from("jcel"), String::from("jcelay@mgi.com"));
    println!("{}, {}", user2.email, user2.username);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Shorthand
    // let _user4 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    let black: Color = Color(0, 0, 0);
    let erigin: Point = Point(0, 0, 0);
    println!("{}", black.1);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// fn build_user(email: &String, username: &String) -> User {
//     User {
//         active: true,
//         username: username.clone(),
//         email: email.clone(),
//         sign_in_count: 1,
//     }
// }

fn min(x: i32, y: i32) -> i32 {
    if x < y {
        return x;
    } else {
        return y;
    }
}

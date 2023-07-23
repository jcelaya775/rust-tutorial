use std::collections::HashMap;
// use std::{cmp:Ordering, io};
use std::collections::*; // import all from collections
use std::io::{self, Write};

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Is Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Dime");
            5
        }
        Coin::Quarter(state) => {
            println!("State quarter is from {:?}!!!", state);
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let penny = Coin::Penny;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(penny));
    println!("{}", value_in_cents(dime));
    println!("{}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    let res = match six {
        None => 0,
        Some(val) => val,
    };
    println!("{res}");

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other), // catch all other variants
    //                                  // must be put last
    // }
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Does nothing
                 // _ => reroll(),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The max is configured to be {}", max);
    }

    // const X: i32 = 4;
    let coin = Coin::Dime;
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // equivalent ^
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    let hello = "Hello, world!";
    let res = fibo(8);
    println!("{}", res);

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("Blue"), String::from("Red"));
    map.insert(String::from("Yellow"), String::from("Green"));
    println!("{:?}", map.get("Blue"));
    println!("{:?}", map);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn fibo(n: u32) -> u32 {
    assert!(n <= 100, "n must be less than or equal to 100");
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

fn find_duplicate_sort(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut prev = nums[0];
    for num in nums {
        if num == prev {
            return num;
        }
        prev = num;
    }
    return -1;
}

fn find_duplicate_hash(nums: Vec<i32>) -> i32 {
    let mut seen = Vec::new();
    for num in nums {
        if seen.contains(&num) {
            return num;
        }
        seen.push(num);
    }
    return -1;
}

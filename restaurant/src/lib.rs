// use std::collections::*;
// use std::io::{self, Write};
// use std::{cmp::Ordering, io};

// Declare modules
mod back_of_house;
mod front_of_house;
mod temp;

use temp::hi::hello::say_hi;

pub use crate::front_of_house::hosting; // re-exporting

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

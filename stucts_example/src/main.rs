use rand::Rng;
use std::cmp::max;
use std::vec;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated fn, but not method (b/c lacks self)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("rect1 is {:#?}", rect1); // pretty-print (uses Debug trait/format)
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 40,
        height: 45,
    };
    println!("rect2 fits within rect1: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(4);
    println!("{:#?}", square);

    // Test loop
    for i in 0..10 {
        let array: Vec<i32> = generate_random_vector(10);
        let max_profit: i32 = max_profit(&array);
        println!("{i}: {:?}", array);
        println!("max profit: {max_profit}\n")
    }
}

fn max_profit(prices: &Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = 1;
    let mut max_profit: i32 = 0;

    while right < prices.len() {
        let curr_profit: i32 = prices[right] - prices[left];
        if prices[right] > prices[left] {
            max_profit = max(curr_profit, max_profit);
        } else {
            left = right;
        }

        right += 1;
    }

    return max_profit;
}

fn generate_random_vector(size: u32) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..size {
        vector.push(rng.gen_range(0..10));
    }

    return vector;
}

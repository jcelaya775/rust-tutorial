fn main() {
    let mut count = 0;

    // loop labels
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    _while_example();
    for_example();

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // println!("fibo(120) = {}", fibo(120));
    println!("fibo(150) = {}", fibo_iterative(150));
}

fn _while_example() {
    println!("while loop:");
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;

    while idx < a.len() {
        println!("the value is: {}", a[idx]);

        idx += 1;
    }
    println!();
}

fn for_example() {
    println!("for loop:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!();
}

fn fibo(n: u128) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }

    return fibo(n - 1) + fibo(n - 2);
}

fn fibo_iterative(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;

    for _ in 2..n {
        let c = b;
        b += a;
        a = c;
    }

    return b;
}

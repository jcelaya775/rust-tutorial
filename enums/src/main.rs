#[derive(Debug)]
struct Ipv4Addr {
    // --snip--
}

#[derive(Debug)]
struct Ipv6Addr {
    // --snip--
}

#[derive(Debug)]
enum IpAddrKind {
    V4(Ipv4Addr), // can put any kind of data inside an enum variant (even another enum)
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
        println!("{:?}", self);
    }
}

fn main() {
    // let four: IpAddrKind = IpAddrKind::V4;
    // let six: IpAddrKind = IpAddrKind::V6;

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let home = IpAddrKind::V4(127, 0, 0, 1);
    // let loopback = IpAddrKind::V6(String::from("::1"));
    // println!("{:?}", home);
    // println!("{:?}", loopback);
    let mine: Message = Message::Write(String::from("hello there charles"));
    mine.call();
    // println!("{}", mine.);

    // let absent_number: Option<i32> = Option::Some(3); // equivalent (Option included in prelude)
    let absent_number: Option<i32> = Some(3);
    println!("{}", absent_number.unwrap());
}

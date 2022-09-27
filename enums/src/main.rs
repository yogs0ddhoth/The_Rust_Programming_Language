#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // dataless variant
    Move { x: i32, y: i32 },    // named fields
    Write(String),              // single value tuple (String)
    ChangeColor(i32, i32, i32), // tuple (i32)
}
// best not to make an enum you won't use all of
impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("x: {x}, y: {y}"),
            Message::Write(str) => println!("{str}"),
            Message::ChangeColor(x, y, z) => println!("x: {x}, y: {y}, z: {z}"),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    dbg!(home);

    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(loopback);

    let m = Message::Write(String::from("Hiya Chuck."));
    m.call();

    let some_number = Some(5);
    
    if some_number.is_some() {
        println!("some number: {}", some_number.unwrap());
    }
}

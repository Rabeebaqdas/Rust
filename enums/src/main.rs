enum IpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,
    Message {x:u32, y:u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[derive(Debug)]
enum UsState {
    Aleska,
    Alabama,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
impl Message{
    fn some_message() {
        println!("Hello world!");
    }
}


fn main() {
    println!("Hello, world!");

    let localhost = IpAddrKind::V4(1, 2, 3, 4);
    let a = Some(5);
    let b = Some("hello world");
    let c: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);  // or None
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(six);
    println!("none = {:?}", six);
    match five {
        Some(5) => println!("five = 5"),
        _ => (),
    }

    if let Some(5) = five{
        println!("five = 5");
    }

    value_in_cent(Coin::Quarter(UsState::Aleska));
}
fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None
    }
}

fn  value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("State quarter form {:?}", state);
            25
        },
    }
}
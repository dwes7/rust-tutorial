enum IpAddrKind {
    V4(u8, u8, u8, u8), // can assosiate an enum field with a type and assign directly without using a separate struct
    V6(String),
}

struct IpAddr {
    kind : IpAddrKind,
    address : String, 
}


// enum with 4 different types with its variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
    Arizona, 
    // --snip-- 
}
enum Coin{
    Penny, 
    Nickel,
    Dime, 
    Quarter(UsState),
}

// The match operator

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quater from {:?}!", state);
            25
        }
    }
}
// We can even implement functions for an enum
impl Message {
    fn call(&self) {
        //method body to call when message is received.
        println!("Message Call!");

    }
}

fn plus_one(x : Option<i32> ) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;


    // let route = IpAddr {
    //     kind: four,
    //     address : String::from("127.0.0.1"),
    // }

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let syntax
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

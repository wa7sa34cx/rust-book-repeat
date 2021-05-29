pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // ---

    let m = Message::Write(String::from("hello"));
    m.call();

    /*
     * The Option Enum
     */

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    // let x = some_number + absent_number;

    /*
     * The match Control Flow Operator
     */
    let coin = Coin::Penny;
    let coin_value = value_in_cents(coin);
    println!("{}", coin_value);

    value_in_cents(Coin::Quarter(UsState::Texas));

    /*
     * Matching with Option<T>
     */
    println!();
    let five = Some(5);
    println!("{:?}", five);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    /*
     * Concise Control Flow with if let
     */
    // let some_u8_value = Some(0u8);
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    if Some(3) == some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let state = UsState::Texas;
    let coin = Coin::Quarter(state);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("do something...");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

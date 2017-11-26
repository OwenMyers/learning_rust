#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Vermont,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin{
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //&Coin::Quarter(_) => 25,
        Coin::Quarter(state) => {
            println!("State quarter is from {:?}", state);
            25
        },
    }
}

fn main() {
    //let my_coin = Coin::Penny;
    let my_coin = Coin::Quarter(UsState::Vermont);
    let my_coin_value: u32;
    my_coin_value = value_in_cents(my_coin);

    println!("The value of my coin is: {}", my_coin_value);

    println!("*****************************");
    let some_u32_v: u32 = 4;
    let some_u32_v2 = 5u32;
    println!("some_u32_v: {}", some_u32_v);
    println!("some_u32_v2: {}", some_u32_v2);

    let some_u8_value = 0u8;
    println!("some_u8_value: {}", some_u8_value);
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}


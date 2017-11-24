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
}


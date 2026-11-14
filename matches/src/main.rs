fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Quarter(UsState::Maryland));
    value_in_cents(Coin::Quarter(UsState::Arizona));
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        }
        Coin::Nickel => {
            println!("This is a nickel");
            5
        }
        Coin::Dime => {
            println!("This is a dime");
            10
        }
        Coin::Quarter(state) => {
            println!("This is a quarter from {state:?}!");
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    Maryland,
    California,
    Mississipi,
    Arizona,
    Washington,
}
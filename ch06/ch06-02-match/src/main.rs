fn main() {
    let penny = value_in_cents(Coin::Penny);
    println!("{penny}");

    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{quarter}");

    match_with_option();
    underbar();
}

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn match_with_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            // Rust knows that we didn’t cover every possible case, and even knows which pattern we forgot!
            // If we comment out None, then error occur
            // error[E0004]: non-exhaustive patterns: `None` not covered
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn underbar() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

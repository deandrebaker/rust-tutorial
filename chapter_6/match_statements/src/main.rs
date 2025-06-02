fn main() {
    let coin = Coin::Nickel;
    let cents = value_in_cents(coin);
    println!("The value of the coin is {} cents.", cents);

    let coin2 = Coin::Penny;
    let cents2 = value_in_cents(coin2);
    println!("The value of coin2 is {} cents.", cents2);

    let coin3 = Coin::Quarter(UsState::California);
    let cents3 = value_in_cents(coin3);
    println!("The value of coin3 is {} cents.", cents3);

    let five = Some(5);
    println!("Five is {:?}", five);
    let six = plus_one(five);
    println!("Six is {:?}", six);
    let none = plus_one(None);
    println!("None is {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Add fancy hats!"),
        7 => println!("Remove fancy hats!"),
        other => println!("Move player to space {}", other),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Add fancy hats!"),
        7 => println!("Remove fancy hats!"),
        _ => println!("Reroll"),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => println!("Add fancy hats!"),
        7 => println!("Remove fancy hats!"),
        _ => (),
    }
}

#[derive(Debug)]
enum UsState {
    Washington,
    California,
    NewYork,
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
            println!("Luckky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
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

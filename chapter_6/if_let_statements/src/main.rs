fn main() {
    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => (),
        }
    }

    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
    }

    {
        let coin = Coin::Quarter(UsState::NewYork);
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quater form {state:?}!"),
            _ => count += 1,
        }
    }

    {
        let coin = Coin::Quarter(UsState::NewYork);
        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quater form {state:?}!");
        } else {
            count += 1;
        }
    }

    {
        let coin = Coin::Quarter(UsState::NewYork);
        match describe_state_quarter(coin) {
            Some(description) => println!("{description}"),
            None => println!("this is not a quarter"),
        }

        let coin = Coin::Dime;
        match describe_state_quarter(coin) {
            Some(description) => println!("{description}"),
            None => println!("this is not a quarter"),
        }
    }
}

#[derive(Debug)]
enum UsState {
    Washington,
    California,
    NewYork,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Washington => year >= 1889,
            UsState::California => year >= 1850,
            UsState::NewYork => year >= 1788,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

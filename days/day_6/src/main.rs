#[derive(Debug)]
enum UsState {
    Alabama,
    Oklahoma,
    Texas,
    Wyoming,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Oklahoma => year >= 1907,
            UsState::Texas => year >= 1845,
            UsState::Wyoming => year >= 1890,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn describe_state_coin(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn print_state_coin(description: Option<String>) {
    if let Some(description) = description {
        println!("{}", description);
    }
}

fn main() {
    print_state_coin(describe_state_coin(Coin::Quarter(UsState::Alabama)));
    print_state_coin(describe_state_coin(Coin::Nickel));
    print_state_coin(describe_state_coin(Coin::Quarter(UsState::Oklahoma)));
    print_state_coin(describe_state_coin(Coin::Dime));
    print_state_coin(describe_state_coin(Coin::Quarter(UsState::Texas)));
    print_state_coin(describe_state_coin(Coin::Penny));
    print_state_coin(describe_state_coin(Coin::Quarter(UsState::Wyoming)));
}
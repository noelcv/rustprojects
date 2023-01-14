#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Penny);
    let nickel_value = value_in_cents(Coin::Nickel);
    println!("The nickel value is {}", nickel_value);
    
    
    let alabama_quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("The alabama quarter value is {}", alabama_quarter);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //pattern => code to run
        //arms are separated by a comma, 
        //unless you have multiple lines of code within curly brackets
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //"match arms can bind to the parts of the value that match the pattern"
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

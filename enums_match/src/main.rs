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
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six variable is {:?}", six); //prints: Six variable is Some(6)
    println!("None variable is {:?}", none); //prints: None variable is None
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    //matches need to be exhaustive, i.e, cover all possible cases
    match x {
        None => None,
        Some(i) => Some(i + 1), //the i is the value that is bound to the Some variant
    }
}
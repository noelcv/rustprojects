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
    
    //Matches and Ownership
    let opt: Option<String> = Some(String::from("Hello World")); //plain enum - type Option<String> 
    match opt {
        Some(_) => println!("Some"), //prints: Some("Hello World")
        None => println!("None")
    }
    
    //this doesn't work as the type is ot a reference `&Option<String>`
    // match opt {
    //     Some(s) => println!("Some"), //her the opt is moved...
    //     None => println!("None")
    // }
    
    println!("{:?}", opt); //...so this is not allowed
    
    //the idiomatic way is to use a reference, so no moved happens
    //Some(s) will just grab a reference to the inner field &String
    match &opt {
        Some(s) => println!("Some: {}", s), //prints: Some: Hello World
        None => println!("None!")
    };
    println!("&opt= {:?}", opt); //&opt= Some("Hello World")
    
    
    //IF LET -- or how to reduce verbose
    
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Max is {}", max),
    //     _ => (),
    // }
    
    //the above can be reduced to:
    //a pattern and an expression separated by an equal sign
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }
    
    //we can also use if let with else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    //instead of...
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }
    
    //we can have it like this...
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
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
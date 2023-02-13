use std::io;
use rand::Rng; //run 'cargo doc --open' to read the docs on the crate
use std::cmp::Ordering;


pub struct Guess {
    value: i32, //private field
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1, got {},", value);
        };
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value)
        }
        Guess { value }
    }
    
    //getter method because the value field of Guess is private
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn make_a_guess(input: &str) -> Guess {
    let guess: i32 = input.trim().parse().expect("Please, type a number!");
    Guess::new(guess)
} 
#[cfg(test)]
mod tests {
    use super::*;  
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be greater or equal to 1")]
    fn smaller_than_1() {
        Guess::new(0);
    }
}
    
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // we're commenting it out to not ruin the fun! :D
    // println!("The secret number is {secret_number}");
    
    loop {
        println!("Please input your guess!!");
        let mut guess = String::new(); //:: denotes an associated function, aka a function implemented on a type
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        //this line will shallow the original guess variable
        //trim removes the whitespace \r\n
        //parse will convert the string to another type, in this case, u32
        //because parse returns a Result type, we use expect to handle the possibility of an error
        //in case you want to compare a number with a symbol or anything like that...
        //note that if the parsed number is Ok, expect will return it.
        // let guess: u32 = guess.trim().parse().expect("Please, type a number!");
        
        //now the correct way with Error Handling like try catch in JS
        //this will ignore a non number input and continue asking for guesses instead of crashing
        //the underscore is a catchall value, it means that we want to match all Err values,
        //but we don't care about the specific error value
        //by switching expect for a match, we can handle the error instead of just having it crashing;
        
        // let guess: u32 = match guess.trim().parse(){
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };
        
        // println!("You guessed:{guess}");
        
        
        //compare the guess with the reference to the secret number
        //like a switch, will break when it finds a suitable matching condition, unless it's a loop
        //like it it here, so we add a break statement, once the user guesses the numbrer
        // match guess.cmp(&secret_number) {
            //     Ordering::Less => println!("Too small!"),
            //     Ordering::Greater => println!("Too big!"),
            //     Ordering::Equal => {
                //         println!("You win!");
                //         break;
                //     }
                // }
                
        //now we're going to use a struct to encapsulate the guess
        let filtered_guess = make_a_guess(&guess);
        println!("You guessed:{}", filtered_guess.value);
        
        match filtered_guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
        
}

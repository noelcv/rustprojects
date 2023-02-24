
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>, //represents the shirts in stock
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) 
        //1. we pass an closure without arguments to unwrap_or_else, but the whole closure expression is `|| self.most_stocked()`
        //2. unwrap or else returns an Option<T>, that can either be None or Some(T)
        //3. if the Option<T> is Some(T), then the value is returned
        //4. However, if Option is None, unwrap_or_else calls the closure `|| self.most_stocked()` and returns the value from the closure
        //the closure captures a immutable reference to the inventory and evaluates it within the body of unwrap_or_else
    }
    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        println!("Counting shirts in stock...");
        for color in &self.shirts {
            println!("Current shirt: {:?}", color);
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
            println!("Red: {:?}", num_red);
            println!("Blue: {:?}", num_blue);
        }
        println!("Finished counting shirts in stock with...");
        println!("Red: {:?}", num_red);
        println!("Blue: {:?}", num_blue);
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("User with preference '{:?}' gets a '{:?}' shirt", user_pref1, giveaway1 );
    
    let user_pref2 = None;   //enum Option<T> { None, Some(T) }
    let giveaway2 = store.giveaway(user_pref2);
    println!("User with preference '{:?}' gets a '{:?}' shirt", user_pref2, giveaway2);
}

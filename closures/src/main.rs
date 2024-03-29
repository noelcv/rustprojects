use std::time::Duration;
use std::thread;

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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
    
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };
    
    println!("Result: {:?}", expensive_closure(30));
    //calculating slowly...
    //after 3 seconds
    //Result: 30
    
    let example_closure = |x| x; // the type of x is inferred from the type of the argument
    let s = example_closure(String::from("hello")); //At this point the compiler will infer that x is of type String...
    println!("s: {:?}", s); //s: "hello"
    
    // and this will then fail...
    // let n = example_closure(5); //expected struct `String`, found integer - mismatched types
    let n = example_closure(5.to_string()); //this will work
    println!("n: {:?}", n); //n: "5"
    
    let flush_closure = |_| ();
    let some_string = String::from("hello");
    println!("flush_closure: {:?}", flush_closure(some_string)); //prints: flush_closure: ()  - the value is dropped immediately std::mem::drop

    let list = vec![1, 2, 3];
    println!("list before closure:  {:?}", list);
    
    // a variable can bind to a closure for later reference
    let only_borrows = || println!("From closure: {:?}", list);
    
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    //CAPTURING Mutable References
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before definining closure, {:?}", list);
    //prints: Before definining closure, [1, 2, 3, 4, 5]

    let mut borrows_mutably = || list.push(7);
    //a print here wouldn't be allowed - between closure definition and closure call, because a
    //mutable borrow is in place
    borrows_mutably();
    println!("After calling closure - borrows_mutably(): {:?} ", list);
    //prints: After calling closure - borrows_mutably(): [1, 2, 3, 4, 5, 7]

    let list = vec![10, 20, 30];
    println!("List before defining closure within new thread: {:?}",list);

    //MOVE - we use it to transfer ownership of list to the new thread
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    
    let mut list = [
        Rectangle { width: 30, height: 50 },
        Rectangle { width: 10, height: 40 },
        Rectangle { width: 60, height: 45 },
        Rectangle { width: 95, height: 35 },
    ];
    
    
    // sort_by_key implements FnMut on the closure -- because it will need to call the closure multiple times: for each element in the slice
    list.sort_by_key(|rect| rect.width);
    println!("Sorted list: {:#?}", list);
    
    
    

    let mut another_list = [
        Rectangle { width: 30, height: 50 },
        Rectangle { width: 10, height: 40 },
        Rectangle { width: 60, height: 45 },
        Rectangle { width: 95, height: 35 },
    ];
    
    
    let mut num_sort_operations = 0;
    
    list.sort_by_key(| rect | {
        //this works because the closure is only capturing a reference that is also being used to perform another operation - increment the counter
        // if we'd try tou push an auxiliar value to a vector, as a way to keep track of the number of operations, we'd get an error as we're trying to move values out of the scope of the closure
        num_sort_operations += 1; 
        rect.width
    });
    
    println!("Sorted list: {:#?}, sorted in {num_sort_operations} operations", list);
}
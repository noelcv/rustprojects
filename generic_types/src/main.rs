fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

//we need to specify not only the generic type, but its capabilities
//in this case, we want to compare values so we use PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//Generics in Structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

//implementing methods on structs with Generics

impl<T> Point<T> {
    //the method x will return a reference to the value in the field x of an instance of the struct Point
    fn x(&self) -> &T {
        &self.x
    }
}

//here we're defining a constraint on the type for instances of Point<f32> only
impl Point<f32> {
    //this method will only be available for instance of Point<f32>
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}  


//"Generic Type parameters in a struct definition aren't always the same as those you use in that same struct's method signatures"
#[derive(Debug)]
struct MyPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

//X1 and Y1 are mentioned as their part of the struct definition
impl<X1, Y1> MyPoint<X1, Y1> {
    //X2, Y2 are only relevant for this method
    fn mixup<X2, Y2>(self, other: MyPoint<X2, Y2>) -> MyPoint<X1, Y2> {
        MyPoint {
            x: self.x,
            y: other.y,
        }
    }
}

#[derive(Debug)]
struct FlexiblePoint<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let numbers_list = vec![34, 50, 25, 100, 65];
    
    let mut largest_number = numbers_list[0];
    
    for number in numbers_list {
        if number > largest_number {
            largest_number = number;
        }
    }
    println!("[NAIVE APPROACH] The largest number is {}", largest_number);
    
    //imagine we would to the same for another list...boring! so we'll make function instead
    
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest_i32(&number_list);
    println!("[with fn largest_i32] the largest_i32 is {}", result);
    
    let another_numbers_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&another_numbers_list);
    println!("[with fn largest_i32] the largest_i32 in another_numbers_list is {}", result);
    //we abstracted code to make it reusable
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("[with fn largest_char] The largest char is {}", result);
    
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("[with fn largest] the largest number is {}", result);
    
    let char_list = vec![ 'y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("[with fn largest] The largest char is {}", result);
    
    let integer = Point { x: 5, y: 10 };
    println!("integer {:?}", integer);
    let float = Point { x: 1.5, y: 2.8 };
    println!("float {:?}", float);
    
    // let mixed_types_wont_work = Point { x: 5, y: 9.8}; //^^^ expected integer, found floating-point number
    // println!{"mixed_types_wont_work {:?}", mixed_types_wont_work};
    
    let mixed_types_can_work = FlexiblePoint { x: 5, y: 9.8};
    println!("mixed_types_can_work with FlexiblePoint<T, U>, {:?}", mixed_types_can_work);
    
    //METHODS
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x()); // here, the method x returns a reference to the value in the field x of the instance p of struct Point
    
    let p1 = MyPoint {x: 5, y: 10.4}; //x has as i32, y has a f64
    let p2 = MyPoint {x: "Hello", y: 'c'}; // x has a string slice (&str), y has a char
    let p3 = p1.mixup(p2); //p3 will have x as i32 any y as char because the method is being called on p1 (self) and p2 as (other)
    println!("[With mixed types from MyPoint] p3.x = {}, p3.y = {}", p3.x, p3.y);
    //prints: [With mixed types from MyPoint] p3.x = 5, p3.y = c
}

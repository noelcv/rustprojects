# Generic Types

[Reference Guide](https://rust-book.cs.brown.edu/ch10-00-generics.html)

Like a function can accept parameters, if can likewise get parameters of any type.

We'll see how to define our types and use them in functions and methods

Generics allow for abstraction at the type level, so that we remove duplication.

Eg. a function that finds the largest number in a list of numbers (i32) will work on any list of i32 numbers

With generics we can make it work on a slice of both `i32` or `char` values.

## In Function Definitions

There are no core methods in Rust. Without restrictions a generic type has no capabilities, so we need to define it in the function signature

```rust
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
```

## In Structs

Like in functions we can specify generic types, but if we want to use multiple types within a struct we need to specify it explicitly:

```rust
//Generics in Structs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct FlexiblePoint<T, U> {
    x: T,
    y: U,
}
```

## In Enums

```rust
//from the standard Library - it allows to express the abstraction of an optional value,i.e, it might have something, or not.
enum Option<T> {
    Some(T), //Some will hold one value of type T
    None, //will hold no value
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Let's refactor this function from the enums match project. If you want to run the code, `cd enums_match && cargo run`.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    //matches need to be exhaustive, i.e, cover all possible cases
    match x {
        None => None,
        Some(i) => Some(i + 1), //the i is the value that is bound to the Some variant
    }
}

//we can modify the function signature to adjust the necessary traits. By using the AddAssign<i32>, we'll allow to sum 1 to the value passed into Some if it has one.
fn plus_one_generic<T: std::ops::AddAssign<i32> + Copy>(x: Option<T>) -> Option<T> {
    match x {
        None => None,
        Some(mut i) => {
            i += 1;
            Some(i)
        },
    }
}

fn main() {
    let five = Some(5);
    let generic_six = plus_one_generic(five);
    println!("Generic six is {:?}", generic_six);
    //prints: Generic six is Some(6)
}
```

## In Methods

when implementing generics on Methods, we have to put it right after the `impl` keyword to inform the compiler we'll be using methods with generics on a type defined by a struct.

```rust
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
        (self.x.powi(2) + self.y.powi(2)).sqrt() //powi() is a method that raises a number to a specified power. Only available for f32 and f64
    }
}  
```

>**"You cannot simultaneously implement specific and generic methods of the same name this way. "**
that's why we have to expand on the signature of functions and methods providing the capabilities required for particular operations that may not be available for all the types.
>**"Rust does not have inheritance-like mechanisms for specializing methods as you might find in an object-oriented language,"** except for Traits

```rust
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

fn main() {
    //METHODS
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x()); // here, the method x returns a reference to the value in the field x of the instance p of struct Point
    
    let p1 = MyPoint {x: 5, y: 10.4}; //x has as i32, y has a f64
    let p2 = MyPoint {x: "Hello", y: 'c'}; // x has a string slice (&str), y has a char
    let p3 = p1.mixup(p2); //p3 will have x as i32 any y as char because the method is being called on p1 (self) and p2 as (other)
    println!("[With mixed types from MyPoint] p3.x = {}, p3.y = {}", p3.x, p3.y);
    //prints: [With mixed types from MyPoint] p3.x = 5, p3.y = c
}
```

Because of *Monomorphization* the generic code is turned into specific code **at compile time**, so there are no performance costs associated with using generic code to prevent duplication.


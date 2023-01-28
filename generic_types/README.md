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

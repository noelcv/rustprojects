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

## In Enums
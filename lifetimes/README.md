# Lifetimes

[Reference Guide](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

For code annotations, `cd src/main.rs && cargo run`

Every reference in Rust has a lifetime - its scope of validity.

To avoid dangling references, or referencing pieces of memory that no longer exist.
Visual representations from the Book. (Listing 10-17 / Listing 10-18) 

```rust

fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

```

Listing 10-18

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

This exercise of comparing the lifetimes is performed by the borrow checker at compile time.

>**"Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes."**

1. Lifetime Parameters - Input
2. Output Lifetimes
3. Output Lifetimes

These rules apply to `fn` and `impl` block. If the compiler cannot satisfy one of these rules, the code won't compile.

1. The compiler assigns a lifetime to each parameter that's a reference:

   ```rust
   fn foo<'a>(x: &'a i32) {...}
   
   fn bar<'a, 'b>(x: &'a i32, y: &'b i32) {}
   ```

2. "If there is only one input lifetime parameter, that lifetime is assigned to all output lifetime parameters"

   ```rust
   fn foo<'a>(x: &'a i32) -> &'a i32 {...}
   ```

3. When having multiple parameters but one of them is `&self` or `&mut self` (methods), "the lifetime of `&self` is assigned to all output lifetime parameters".



## Static Lifetimes

A special annotation that binds the lifetime of a data structure to the duration of the program, and hence makes it always available.
Needs to be used sparely.

```rust
let some_string_literal: &'static str = "I have a static lifetime";
```

## Wrap it up

```rust
use std::fmt::Display;

//putting it all together: Generic Type Parameters, Trait Bounds and Lifetimes;
//this function will print the announcement and return the longest string slice from either x or y
//<'a, T> - lifetime is a kind of generic, so we pass it within angle brackets after the function name
fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display, //the where clause specifies that the generic type `T` must implement the `Display` trait...
    {
        println!("Announcement! {}", ann); //...because it will be needed here to print the announcement within the {}
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
fn main() {
  let wrap_it_up = longest_with_an_announcement("This is the X string slice", "This is the Y string slice - let's make it longer", "This is an announcement in the form of a string slice, which implements the Display trait and therefore can be printed in curly braces!"); 
  //prints: Announcement! This is an announcement in the form of a string slice, which implements the Display trait and therefore can be printed in curly braces! 
  println!("wrap_it_up should be the longest string slice from the previous call: {}", wrap_it_up);
  //prints: wrap_it_up should be the longest string slice from the previous call: This is the Y string slice - let's make it longer
}
```

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

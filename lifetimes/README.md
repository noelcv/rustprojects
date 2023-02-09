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
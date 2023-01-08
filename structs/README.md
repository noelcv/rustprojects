# Structs

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 5](https://rust-book.cs.brown.edu/ch05-00-structs.html) of The Book.

At first glance, structs are the equivalent of Interfaces in TypeScript. They can have their own methods.



## Ownership of Struct Data
We want each instance of the struct to own its data for as long as the struct is valid.
Therefore, we use type `String` instead of `&str`. Keeping references to data owned by something else is possible, but require *lifetimes* (see chapter 10)

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
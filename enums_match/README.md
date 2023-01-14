# Enums - The Match Control Flow

To explore the code with annotations open `src/main.rs`

Check the context of these annotations from [Chapter 6.2](https://rust-book.cs.brown.edu/ch06-02-match.html) of The Book.

Quick Notes

- The purpose of the `match` keyword is "to compare a value against a series of patterns and then execute code based on which pattern matches." - which is basically what a switch / case would look like at first glance in Javascript.

- It's like a coin-sorting machine, with holes. The first fitting hole will let the coin in.

- While a `if` statement returns a boolean a `match` can return any type

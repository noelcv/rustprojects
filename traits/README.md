# Traits

[Reference Guide Chapter 10.2 Traits: Defining Shared Behavior](https://rust-book.cs.brown.edu/ch10-02-traits.html)

At first glance, traits are like Interfaces (TypeScript), but different as they define shared behavior generically, and then each type needs to implement its particular implementation.

For a complete example, go to `cd src/main.rs` and `cargo run`


## Orphan Rule

To ensure coherence, **"we can implement a trait on a type only if at least one type is local to our crate"**.

e.g implement trait `Summary` on `Vec<T>` because Summary is defined on our example library, but not `Display` on `Vec<T>` because neither are local, but rather from the standard library, which doesn't belong to us.

Why? to ensure we can't break other people's code or vice versa.

## Default implementations

>***"The syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation."***

Imagining that we expand the default behavior of the Summary trait we can than implement the default behavior with an empty `{}` block

```rust
impl Summary for NewsArticle {}
```

We can expand on default implementations, but "it isn't possible to call the default implementation from an overriding implementation of that same method"

E.g. in the example, to keep the previous functionality on the articles, we had to refactor the implementation for the NewsArticle to override the new default implementation on the `Summary` trait to calling a `summarize_author()`. This is mostly for illustration purposes, though.

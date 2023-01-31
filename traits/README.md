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

## Using Traits as Parameters

You can either use the `impl Trait` syntax or *trait bound* syntax. The first is syntactic sugar for the second, and while the first is convenient when our requirements are loose, if you need to express more complex and strict rules go for the second one. Here are examples, but for a full overview `cd traits_parameters && cargo run`

```rust
//Using Traits as Parameters in functions that accept any type that implements a Trait
//using `impl trait` syntax aka syntax sugar for the more verbose syntax below
pub fn notify(item: &impl Summary) {
    println!("[NOTIFY: BREAKING NEWS], {}", item.summarize());
}


//using the trait bound syntax
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("[NOTIFY VERBOSE BREAKING NEWS], {}", item.summarize());
}

//While here the `impl trait` syntax allows for any type that implements the Summary trait to be passed as an argument to the function...
pub fn notify_multiple(item1: &impl Summary, item2: &impl Summary) {
    println!("[NOTIFY MULTIPLE: BREAKING NEWS 1], {}", item1.summarize());
    println!("[NOTIFY MULTIPLE: BREAKING NEWS 2], {}", item2.summarize());
}


//...here the generic type T used trait bound syntax requires / constrains that both arguments are of the same type and implement the Summary trait
pub fn notify_multiple_strict<T: Summary> (item1: &T, item2: &T) {
    println!("[NOTIFY MULTIPLE STRICT: BREAKING NEWS 1], {}", item1.summarize());
    println!("[NOTIFY MULTIPLE STRICT: BREAKING NEWS 2], {}", item2.summarize());
}
```

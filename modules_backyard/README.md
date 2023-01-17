## Modules

[Reference Guide](https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html)
***

- *paths* - to name things
- `use` - to bring a path into scope
- `pub` - to make items public
- `as`- to cast?
- *glob operator* - tbd

***

1. Start from the crate root - `lib.rs`
2. Declare modules in the crate root - e.g.: `mod garden`
3. Declare submodules outside the crate root - e.g.: `src/garden.rs` -> `mod vegetables`
4. Paths to code in modules (as the privacy allows) - e.g.: `crate::garden::vegetables::Asparagus`
5. Private vs Public - e.g: `pub mod` instead of `mod` (because code is private by default inside module to parent)
6. Shortcuts for long path: e.g.: `use crate::garden::vegetables::Asparagus` -> then you can refer to `Asparagus`  
  
```md
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Under the restaurant lib crate initiated with `cargo new restaurant --lib`, we have nested modules as represented in the following *module tree*:

```md
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Although using modules has no impact at runtime, but rather at compile time, splitting it like that makes it easier to navigate in larger codebases, just like in TypeScript, for instance.

***

## Paths

- ***Absolute paths*** -> starts from the `crate`
- ***Relative paths*** -> starts from current module, and uses `self`, `super` or other identifier

Child modules can see the context they're defined, but hiding inner implementation details is the default.So, parents cannot see or don't have access to those details unless they're public.

>*"making the module public doesn’t make its contents public"*

>*"The privacy rules apply to structs, enums, functions, and methods as well as modules"*

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        
        fn serve_order() {}
        
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    //relative path
    front_of_house::hosting::add_to_waitlist();
}

```

More info at [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

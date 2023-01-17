# Packages and Crates

To explore the code with annotations open `src/main.rs`

Check the context of these annotations from [Chapter 7](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) of The Book.

Quick Ref Guide from the Book

> - **Packages**: A Cargo feature that lets you build, test, and share crates
> - **Crates**: A tree of modules that produces a library or executable
> - **Modules** and **use**: Let you control the organization, scope, and privacy of paths
> - **Paths**: A way of naming an item, such as a struct, function, or module

>*"Crate is the smallest amount of code that the Rust compiler considers at a time":*

- **Bin** crates: need a main function, can be executed
- **Lib** crates: no main function, for shared functionality

>*"A package is a bundle of one or more crates that provides a set of functionality":*

- **Cargo.toml**: to define how to build the crates
- A package can have many bins but only one lib, and at least, one crate.
  
>*"Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package"*

If a `lib.rs` is present, the compiler will know that is the crate root.

>*"The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate"*

A `package` with `src/main.rs` (bin) and `src/lib.rs` (lib) has two crates. We can have multiple bin files under `src/bin`

```md
foobar
├── Cargo.toml
├── build.rs
└── src/
    ├── main.rs
    ├── util.rs
    ├── lib.rs
    └── bin/
        └── alt.rs
```

- `main.rs` -> binary crate
- `lib.rs` -> library crate
- `bin.rs` -> library crate
- `util.rs` -> module
- `build.rs` -> build script


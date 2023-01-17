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

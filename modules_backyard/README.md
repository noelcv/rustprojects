# Modules

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
  
  - the `super` keyword is great for logic that most likely will remain closer in scope, and if it is moved, most likely will be moved together.

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
    
    //order a breakfast in the summer with an a whole wheat toast
    let mut meal = back_of_house::Breakfast::summer("whole wheat");
    
    //Change our mind
    meal.toast = String::from("bagel");
    println!("I'd like a {} toast, bitte!", meal.toast);
    
    //the next line wouldn't compile because seasonal_fruit is private
    //meal.seasonal_fruit = String::from("pineapple");
}

```

More info at [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Making Structs and Enums Public

we keep the same logic of public and private fields. A struct may be public, and yet have private fields

If a struct has privates fields, we need a public associated function to construct an instance of it.

```rust
mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        //because Breakfast includes a private field, 
        //we need to provide a public associate function to construct an instance
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

However, in a public enum all the variants are public.

```rust
  pub enum Appetizer {
        Soup,
        Salad,
    }
    
  let order1 = back_of_house::Appetizer::Soup;
```


## `Use` keyword to save the day
Use makes modules available in the scope where is defined and keeps the code dry

```rust
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //specifying the parent module makes it clear where the function comes from
    hosting::add_to_waitlist();
}

//to make it available to the next module use super

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
```

When importing a struct, though, the convention is to specify the full path, unless there is name collision:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

...

use std::fmt;
use std::io;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}
```

## We can also use `as` to cast *alias*

```rust
use::std::fmt::Result;
use::std:io::Result as IoResult;

fn function2() -> IoResult<()> {}

```

## `pub use` to re-export modules, shorten paths or simply expose the code under a different light

It's all about code structure and design for both builders and consumers of the code.

- like in the restaurant analogy
  - we can think of back and front of the house
  - but the clients only think of it it in terms of a restaurant
  - so we can have our code structure and still expose it differently

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//without pub use
pub fn eat_at_restaurant() {
    //assuming restaurant as the name of the crate
    restaurant::front_of_house::hosting::add_to_waitlist()
}

//with pub use we reexport the module to allow for it to be called
//"as if" it's being declared in that scope:
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

```

## External Packages

Like a package.json file, we use Cargo.toml to specify which packages and dependencies we'll be using in a project

- list them in the Cargo.toml file

```toml
  [dependencies]
  `rand = "0.8.3."`
```

- Use the `use` keyword to bring them to scope

```rust
use rand::Rng;

//it comes from the standard collection - no need to include in the Cargo.toml 
use std::collections::HashMap; 

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

- Using nested paths we can bring several modules from a given crate
  
  - In Rust, the equivalent would be:
  
 ```rust
 //instead of separate imports
 use std::cmp::Ordering;
 use std::io;
 //we can group it like this
 use std::{cmp::Ordering, io}
 ```

- In JS/TS, you'd do something like this:

```javascript
  import { readFileSync, writeFileSync } from 'fs'
  import { useEffect, useState } from 'react'
  ```  
  
- Merging paths into one statement
  
```rust
use std::io;
use std::io::Write;

//could be refactored into:
use std::io::{self, Write};
```

### The glob operator ( `*` ) is basically a wildcard to bring everything into scope;

- in Javascript:
  
```javascript
import * from 'fs'
```

- in Rust:

```rust
use std::collections::*;
```

- It shall be used with caution, for both clarity, and to avoid bringing unnecessary modules into scope.

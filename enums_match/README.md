# Enums - The Match Control Flow

To explore the code with annotations open `src/main.rs`

Check the context of these annotations from [Chapter 6.2](https://rust-book.cs.brown.edu/ch06-02-match.html) of The Book.

Quick Notes

- The purpose of the `match` keyword is "to compare a value against a series of patterns and then execute code based on which pattern matches." - which is basically what a switch / case would look like at first glance in Javascript.

- It's like a coin-sorting machine, with holes. The first fitting hole will let the coin in.

- While a `if` statement returns a boolean a `match` can return any type

> *"Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite."*

## Catch-all patterns

Once matches are exhaustive, we always need to cover all the possibilities. One way of doing that is with a catch-all pattern, like the one in the example provided in The Book. :

```rust
let dice_roll = 9;
match dice_roll {
  3 => add_fancy_hat();
  7 => remove_fancy_hat();
  other => move_player(other);
  
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

In the example above, other will be passed to the move_player function when the roll_dice is not 3 or 7.

If we wouldn't want to use the value we could use the `_` pattern.

```rust
let dice_roll = 9;
match dice_roll {
  3 => add_fancy_hat();
  7 => remove_fancy_hat();
  _ => reroll();  

}
...
fn reroll() {}
```

When using an empty tuple, we are explicitly telling Rust that we are only interested in the patterns provided, and in an absence of such a match, the code shall not run.

```rust
match dice_roll {
  3 => add_fancy_hat();
  7 => remove_fancy_hat();
  _ => (); 
}
```

### `If Let` is Syntatic sugar for a `match` when a value matches a pattern and ignores the rest

```rust

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("Max is {}", max),
    //     _ => (),
    // }
    
    //the above can be reduced to:
    //a pattern and an expression separated by an equal sign
    if let Some(max) = config_max {
        println!("Max is {}", max);
    }
    
    //we can also use if let with else
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    //instead of...
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1,
    // }
    
    //we can have it like this...
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
```

#### Pros and Cons

- Less typing
- Less indentation
- Less typing
- but also less exhaustive checking

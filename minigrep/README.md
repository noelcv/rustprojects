# MinigrepIO - A project based on Chapter 12

`cargo run -- needle haystack`

```bash
[src/main.rs:5] args = [
    "target/debug/minigrep", //is the name of our binary - &args[0]
    "needle",
    "haystack",
]
```

```bash
➜  minigrep git:(main) ✗ cargo run -- needle haystack
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 1.19s
     Running `target/debug/minigrep needle haystack`
Searching for needle
In file haystack
```
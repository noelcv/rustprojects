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

We added the `poem.txt` at the root of the project,

```bash
➜  minigrep git:(main) ✗ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (/Users/noel/rustprojects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text: 
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

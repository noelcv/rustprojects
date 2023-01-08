# Ownership

To explore the code with annotations open `src/main.rs`
Check the context of these annotations from [Chapter 4.1](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html) of The Book.

## The Heap and the Stack

like a restaurant

restaurant -> heap

group of friends -> fixed, known size

staff -> memory allocator

table for the group -> slot in memory

late friends ask where your group is seated -> pointer

## The 3 rules of Ownership

### 1. Each value has a owner

### 2. Only one owner at a time

### 3. Once a owner goes out of scope, the value is dropped.


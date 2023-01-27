# Generic Types

[Reference Guide](https://rust-book.cs.brown.edu/ch10-00-generics.html)

Like a function can accept parameters, if can likewise get parameters of any type.

We'll see how to define our types and use them in functions and methods

Generics allow for abstraction at the type level, so that we remove duplication.

Eg. a function that finds the largest number in a list of numbers (i32) will work on any list of i32 numbers

With generics we can make it work on a slice of both `i32` or `char` values.


use std::env; //bring it to scope so we can use args()

fn main() {
    let args: Vec<String> = env::args().collect(); // turn the iterator into a collection of values, in this case a Vector of strings
    dbg!(args);
}


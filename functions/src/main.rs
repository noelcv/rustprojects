//This is a comment
fn main() {
    //most common place to leave a comment
    println!("Hello, world!");
    print_labeled_measurements(10, 'h'); //a comment here is also acceptable, but less common
    let y = five();
    let n = plus_one(5);
    println!("the value of y is {y}");
    println!("the value of n is {n}");
    another_function(5);
}

// if a comment requires several lines
// we need multiple slashes
// Funny enough, these come automatically
// which is fantastic! 
fn another_function(x: i32) {
    println!("The value of x is: {x}.");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("Inside print_labeled_measurements: value - {value}, unit_label - {unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

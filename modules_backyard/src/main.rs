//bring the Asparagus into scope
use crate::garden::vegetables::Asparagus;

//tell the compile to include the code in src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {color: String::from("green"), height: 1.09};
    println!("Hi! I'm a {:#?}'!", plant);
}

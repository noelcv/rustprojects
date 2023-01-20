mod front_of_house;
mod back_of_house;

//Re-exporting with pub use
//we bring to this scope the hosting module and make it accessible for other modules to call it
//"as if it had been defined in that code's scope"
pub use crate::front_of_house::hosting;
pub use crate::back_of_house::back_of_house::{Breakfast, Appetizer};

pub fn eat_at_restaurant() {
    //absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    
    //relative path
    // front_of_house::hosting::add_to_waitlist();
    
    //with pub use
    hosting::add_to_waitlist();
    
    //order a breakfast in the summer with an a whole wheat toast
    let mut meal = Breakfast::summer("whole wheat");
    
    //Change our mind
    meal.toast = String::from("bagel");
    println!("I'd like a {} toast, bitte!", meal.toast);
    
    //the next line wouldn't compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("pineapple");
    
    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}

//front_of_house and eat_at_restaurant are siblings

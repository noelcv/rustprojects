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
    // meal.seasonal_fruit = String::from("pineapple");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//front_of_house and eat_at_restaurant are siblings


fn deliver_order() {
    println!("Et voilà...bon appétit!");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //here we use super to refer to the parent module scope
        //good to use when we know that similar logic will remain closer, and if they are to move,
        //will move together
        super::deliver_order();
    }
    
    fn cook_order() {
        println!("Cooking order...");
    }
    
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
    
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
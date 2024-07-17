mod front_of_house;

// 'use' keyword, basically a shortcut to a path
// use crate::front_of_house::hosting;
// idiomatic 'use' paths, use with structs, enums and others
// use crate::front_of_house::hosting::add_to_waitlist;
// Provide new names (aliases) with 'as'
use std::io::Result as IoResult;
// Re-exporting names with 'pub use'
// pub use crate::front_of_house::hosting;
// Clean up large 'use' lists with nested paths
// use std::{cmp::Ordering, io}; // Multiple from std library
// use std::io::{self, Write}; // Brings std::io and std::io::Write into scope
// use std::collections::*; // Bring all public items

pub fn eat_at_restaurant() {
    // sibling
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // Path from 'use'
    // hosting::add_to_waitlist();

    // direct function from idiomatic 'use'
    // add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {} // in the parent module

mod back_of_house {
    pub enum Appetizer {
        // pub enum == pub variants e.g Soup is public
        Soup,
        Salad,
    }
    pub struct Breakfast {
        // pub struct != pub fields
        pub toast: String,      // specify pub on field if needed, this is public
        seasonal_fruit: String, // this is private
    }

    impl Breakfast {
        // implement struct
        pub fn summer(toast: &str) -> Breakfast {
            // return breakfast for the summer
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // uses 'super' to access item from parent
    }

    fn cook_order() {}
}

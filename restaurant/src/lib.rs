// created with cargo new restaurant --lib

// this is idiomatic
use crate::front_of_house::hosting;
// and then call a fn inside it with hosting::add_to_waitlist();
// use crate::front_of_house::hosting::add_to_waitlist; is not idiomatic
// because when calling it with add_to_waitlist(); you dont know if its a local fn 
// or an imported fn
// so, its idiomatic to not write the full path, go until the parent module of the fn 
// if you have a fn with the same name in two different modules, you should call it 
// like this:
/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/
// another solution would be using aliases:
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/
// both solutions are idiomatic, so use whatever you want

// it is possible to write: 
// pub use crate::front_of_house::hosting;
// so external code can for example call the fn add_to_waitlist with 
// restaurant::hosting::add_to_waitlist() instead of 
// restaurant::front_of_house::hosting::add_to_waitlist()
// this is called re-exporting
// if someone would like to use this API but doesnt know how the previously 
// defined restaurants structure (front_of_house and back_of_house), 
// this could be used to hide that structure, and providing a simpler one

// you should write the full path when bringing things like data structures, 
// for example:
use std::collections::HashMap;

// nested paths: 
// this: 
// use std::cmp::Ordering;
// use std::io;
// is the same as: 
// use std::{cmp::Ordering, io};
// if one is a subpath of the other:
// use std::io;
// use std::io::Write;
// it would end like this:
// use std::io::{self, Write};

// glob operator brings everything:
// use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// this searches the front_of_house file and brings it to scope
pub mod front_of_house;

mod back_of_house {
    // struct fields are by default private, even when the struct itself is public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // all enums variants are public if the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // path to parent's fn
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// if we add mod customer {
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // same as this:
    hosting::add_to_waitlist();
    // because of the "use" keyword

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    // because seasonal_fruit is private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
//}
// then the previous use is out of scope, so you would need another use
// inside customer





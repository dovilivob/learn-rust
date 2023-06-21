// use std module, and rename them to prevent conflict
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

use std::io::{self, Write}; // use nested path to simplify the code below
                            // use std::io::*; // import all public items defined in std::io

fn function_1() -> FmtResult {
    // --snip--
    OK(())
}

fn function_2() -> IoResult<()> {
    // --snip--
    Ok(())
}

// use external module
/*
use rand::Rng;
use rand::ErrorKind::Transient;
use rand::CryptoRng;
*/

use rand::{CryptoRng, ErrorKind::Transient, Rng}; // Use nested path to simplify the code above

// Define a module
mod front_of_the_house;

// define a function in the crate root, to call it in a child module, use "super"
fn serve_order_in_root() {}

mod back_of_the_house {
    // use struct in mod
    pub struct Breakfast {
        // implicitly set "toast" to public
        pub toast: String,
        seasonal_fruit: String,
    }

    // method implementation
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // use enum in mod
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // use super to go to the parent module of back_of_the_house (crate root)
        super::serve_order_in_root();
    }

    fn cook_order() {}
}

// use crate::front_of_the_house::hosting; // ==  use self::front_of_the_house::hosting;
pub use crate::front_of_the_house::hosting; // use "pub use" to re-export a name, which allows external code to use the new name

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // Relative path
    front_of_the_house::hosting::add_to_waitlist();

    // call the public method
    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    // Call the public enum
    let order_1 = back_of_the_house::Appetizer::Soup;
    let order_2 = back_of_the_house::Appetizer::Salad;

    // "use" keyword
    hosting::add_to_waitlist(); // == front_of_the_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // call the function in the external module
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

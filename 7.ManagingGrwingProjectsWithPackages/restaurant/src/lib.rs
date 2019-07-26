#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        pub mod back_of_house {
            pub struct Breakfast {
                pub toast: String,
                seasonal_friute: String,
            }

            pub enum Appetizer {
                Soup,
                Salad,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    return Breakfast {
                        toast: String::from(toast),
                        seasonal_friute: String::from("peaches"),
                    }
                }
            }

            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }

            fn cook_order() {}
        }
    }
}

pub use crate::front_of_house::hosting;
use crate::front_of_house::serving::back_of_house;
use std::collections::HashMap;
use std::collections::*;
use std::io::{self, Write};

pub use crate::front_of_house1::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast pleas", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;

    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}



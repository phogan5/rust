mod front_of_house;

pub use crate::front_of_house::hosting;
pub fn eat_something_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //use path
    hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
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

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we want
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //The next line cant compile because seasonal_fruit is never declared as public
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


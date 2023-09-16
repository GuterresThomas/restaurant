pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

}




mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str, seasonal_fruit: &str) -> Breakfast {
            Breakfast {
                toast: String::from("toast"),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye","Apple");

    meal.toast = String::from("Wheat");
    meal.seasonal_fruit = String::from("Avocato");
    println!("I'd like {} toast and {}, please", meal.toast, meal.seasonal_fruit);
}



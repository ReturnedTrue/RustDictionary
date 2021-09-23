#![allow(unused_parens)]

mod dictionary;

use dictionary::Dictionary;

fn main() {
    let mut dict = Dictionary::<&str, i8>::new();

    dict.set("Player1", 1);
    dict.set("Player2", 1);

    println!("Player1: {}", dict.get("Player1").unwrap());

    let mut mapped_dict = dict.map(|item| item + 1);
    mapped_dict.update("Player1", |item| item + 1);

    let filtered_dict = mapped_dict
		.filter(|item| item == &3);
    filtered_dict.for_each(|key, _value| println!("{} has 3 pts", key));
}

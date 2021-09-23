#![allow(unused_parens)]

mod dictionary;

use dictionary::Dictionary;

fn main() {
    let mut dict = Dictionary::<&str, i8>::new();
    let index = "A";

    dict.set(index, 1);
    dict.set(index, 2);

    println!("Index: {}", dict.get(index).unwrap());

    let mut mapped_dict = dict.map(|item| item + 1);

    mapped_dict.set("B", 2);

    let filtered_dict = mapped_dict.filter(|item| item == &3);

    println!("Updated index: {}", filtered_dict.get(index).unwrap());
}

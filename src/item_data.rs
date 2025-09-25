use std::collections::HashMap;

use crate::item_handeler::Item;
use crate::item_handeler::ItemType;

pub fn build_item_data() -> HashMap<String, Item> {
    #[rustfmt::skip]let mut item_collection = HashMap::from([
        (String::from("Apple"), Item::build_item("apple", 0.25, "Has like vitamins n fibers inside.", ItemType::Food, 1, 1)),
        (String::from("Banana"), Item::build_item("babana", 0.12, "Dont eat to much it has like radiation inside", ItemType::Food, 1, 1))
    ]);
    return item_collection;
}

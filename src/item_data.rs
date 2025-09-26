use std::collections::HashMap;

use crate::item_handeler::Item;
use crate::item_handeler::ItemType;

pub fn build_item_data() -> HashMap<String, Item> {
    #[rustfmt::skip]let item_collection = HashMap::from([
        (String::from("Apple"), Item::build_item("apple", 0.25, "Has like vitamins n fibers inside.", ItemType::Food, 1, 1, None)),
        (String::from("Banana"), Item::build_item("babana", 0.12, "Dont eat to much it has like radiation inside", ItemType::Food, 1, 1, None)),
        (String::from("Wood"), Item::build_item("Wood", 2.5, "You can burn it, you can build with it, you can also probably eat it", ItemType::Recource, 2, 1, None)),
        (String::from("Iron Bar"), Item::build_item("Iron Bar", 20.0, "made of iron...", ItemType::Recource, 2, 1, None)),
        (String::from("M4 Carbine"), Item::build_item("M4 Carbine", 2.92, "Trusted by the US army since '94.", ItemType::Weapon, 3, 2, None)),
        (String::from("Ak-104"), Item::build_item("Ak-104", 3.9, "Works no matter what.", ItemType::Weapon, 3, 2, None)),
    ]);
    return item_collection;
}

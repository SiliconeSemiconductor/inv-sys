use std::collections::HashMap;

use crate::item_handeler::Item;
use crate::item_handeler::ItemType;

pub fn build_item_data() -> HashMap<&str, Item> {
    #[rustfmt::skip]let item_collection: HashMap<&str, Item> = HashMap::from([
        ("Apple", Item::build_item("apple", 0.25, "Has like vitamins n fibers inside.", ItemType::Food, 1, 1)),
        ("Banana", Item::build_item("babana", 0.12, "Dont eat to much it has like radiation inside", ItemType::Food, 1, 1)),
        ("Wood", Item::build_item("Wood", 1.0, "You can burn it, you can build with it, you can probably even eat it.", ItemType::Recource, 1,2)),
        ("Iron_Ingot", Item::build_item("Iron_Ingot", 20.0, "made of iron...", ItemType::Recource, 2, 1)),
    ]);
    return item_collection;
}

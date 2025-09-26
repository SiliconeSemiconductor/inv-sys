use crate::item_handeler::Item;

mod item_data;
mod item_handeler;

fn main() {
    let items = item_data::build_item_data();
    println!("Built item Data");
    for (k, v) in items {
        println!("{k}");
        Item::print_item_extended(&v);
    }
}

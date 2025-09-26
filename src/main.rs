use crate::item_handeler::Item;

mod container_handeler;
mod item_data;
mod item_handeler;

fn main() {
    let items = item_data::build_item_data();
    println!("Built item Data");
    for (k, v) in items {
        println!("{k}");
        Item::print_item_extended(&v)
    }

    container_handeler::Container::build_container(String::from("Chest"), Some(2000.0), 10, 10);
}

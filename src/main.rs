mod item_data;
mod item_handeler;

fn main() {
    let items: std::collections::HashMap<&str, item_handeler::Item> = item_data::build_item_data();
    println!("Built item Data");
    for (k, v) in items {
        println!("{k}");
        item_handeler::Item::print_item_extended(&v);
    }
}

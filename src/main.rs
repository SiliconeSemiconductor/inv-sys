enum ItemType {
    Armor,
    Weapon,
    Food,
    Recource,
    Junk,
}
enum ContainerType {
    None,
    Type(ItemType),
}

struct Item {
    name: String,
    weight: f32,
    description: String,
    item_type: ItemType,
}

struct Container {
    name: String,
    capacity: f32,
    load: f32,
    description: String,
    content: Vec<Item>,
}

impl ItemType {
    fn to_string(&self) -> &str {
        match self {
            ItemType::Armor => "Armor",
            ItemType::Weapon => "Weapon",
            ItemType::Food => "Food",
            ItemType::Recource => "Recource",
            ItemType::Junk => "Junk",
        }
    }
}

impl Item {
    fn print_item(&self) {
        println!(
            "Name: {}\nWeight: {}kg\nDescription: {}\n",
            self.name, self.weight, self.description
        );
    }
    fn print_item_extended(&self) {
        println!(
            "Name: {}\nWeight: {}kg\nDescription: {}\n\nitem_type: {}",
            self.name,
            self.weight,
            self.description,
            self.item_type.to_string(),
        );
    }
    fn build_item(name: &str, weight: f32, description: &str, item_type: ItemType) -> Item {
        Item {
            name: String::from(name),
            weight: weight,
            description: String::from(description),
            item_type: item_type,
        }
    }
}

fn main() {
    let mut items: Vec<Item> = Vec::new();
    items.push(Item::build_item(
        "Apple",
        0.1,
        "Its an apple",
        ItemType::Food,
    ));

    #[rustfmt::skip]items.push(Item::build_item("Jumpable Plate Carrier",0.6,"The JUMPABLE PLATE CARRIER™ (JPC) is a lightweight and minimal vest designed for maximum mobility, weight savings, and packability. At just over one pound for the entire carrier, the JPC™ offers a variety of configuration options to suit an operator’s needs in terms of load carriage and comfort. It features our SKELETAL™ CUMMERBUND system with our patented integrated attachment system that allows pouches to be mounted on both the inside and outside of the cummerbund, shedding unnecessary weight and bulk while improving ventilation. It also features integrated admin and magazine pouches on the front of the carrier. Sizes are based on the size of the ballistic plate. For example, a large plate will fit best in a large JPC™. Patented, see www.lwpatents.com. Made in US from US materials. ",ItemType::Armor,));
    #[rustfmt::skip]items.push(Item::build_item("AK-105", 7.5,"Compact assault rifle packed with mods, optics, and extended mags; heavy but deadly in close to mid-range combat.", ItemType::Weapon));
    #[rustfmt::skip]items.push(Item::build_item("Iron Bar",20.0,"Made of iron.",ItemType::Recource,));
    #[rustfmt::skip]items.push(Item::build_item("Empty Bottle",0.25,"Empty bottle. No more beer :(",ItemType::Junk,));

    for i in items {
        i.print_item();
        i.print_item_extended();
    }
}

fn main_loop() {
    while true {}
}

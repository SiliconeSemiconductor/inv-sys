pub struct Item {
    name: String,
    weight: f32,
    description: String,
    item_type: ItemType,
    dimensions: ItemSize,
}

pub enum ItemType {
    Armor,
    Weapon,
    Food,
    Recource,
    Junk,
}

struct ItemSize {
    width: u8,
    height: u8,
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

impl ItemSize {
    fn define_size(width: u8, height: u8) -> ItemSize {
        ItemSize {
            width: width,
            height: height,
        }
    }
}

impl Item {
    pub fn print_item(&self) {
        println!(
            "Name: {}\nWeight: {}kg\nDescription: {}\n",
            self.name, self.weight, self.description
        );
    }
    pub fn print_item_extended(&self) {
        println!(
            "Name: {}\nWeight: {}kg\nDescription: {}\n\nitem_type: {}",
            self.name,
            self.weight,
            self.description,
            self.item_type.to_string(),
        );
    }

    pub fn build_item(
        name: &str,
        weight: f32,
        description: &str,
        item_type: ItemType,
        item_width: u8,
        item_height: u8,
    ) -> Item {
        Item {
            name: String::from(name),
            weight: weight,
            description: String::from(description),
            item_type: item_type,
            dimensions: ItemSize::define_size(item_height, item_width),
        }
    }
}
// test

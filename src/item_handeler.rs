pub struct Item {
    name: String,
    weight: f32,
    description: String,
    item_type: ItemType,
}

pub enum ItemType {
    Armor,
    Weapon,
    Food,
    Recource,
    Junk,
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

    pub fn build_item(name: &str, weight: f32, description: &str, item_type: ItemType) -> Item {
        Item {
            name: String::from(name),
            weight: weight,
            description: String::from(description),
            item_type: item_type,
        }
    }
}

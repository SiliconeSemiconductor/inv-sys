#![allow(dead_code)]
pub struct Item {
    name: String,
    weight: f32,
    description: String,
    item_type: ItemType,
    dimensions: ItemSize,
    attributes: Option<Vec<ItemAttribute>>,
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
            "Name: {}\nWeight: {}kg\nDescription: {}\nitem_type: {}\nitem_dimensions: {}\nitem_attributes: {}\n",
            self.name,
            self.weight,
            self.description,
            self.item_type.to_string(),
            self.dimensions.to_string(),
            self.attributes_to_string(),
        );
    }

    pub fn build_item(
        name: &str,
        weight: f32,
        description: &str,
        item_type: ItemType,
        item_width: u8,
        item_height: u8,
        item_attribute: Option<Vec<ItemAttribute>>,
    ) -> Item {
        Item {
            name: String::from(name),
            weight: weight,
            description: String::from(description),
            item_type: item_type,
            dimensions: ItemSize::define_size(item_height, item_width),
            attributes: item_attribute,
        }
    }

    fn attributes_to_string(&self) -> String {
        let mut return_string = String::new();
        if let Some(vec) = self.attributes.as_ref() {
            for i in vec {
                return_string.push_str(&i.to_string());
            }
        } else {
            return_string.push_str("No Attributes")
        }
        return return_string;
    }
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

// just a wip but later please add an exaustive handling of atributes
pub struct ItemAttribute {
    attribute_name: String,
    attribute_value: i32,
}

impl ItemAttribute {
    fn to_string(&self) -> String {
        format!(
            "Name: {}, Value: {}",
            self.attribute_name, self.attribute_value
        )
    }
}

pub struct ItemSize {
    width: u8,
    height: u8,
}

impl ItemSize {
    fn define_size(width: u8, height: u8) -> ItemSize {
        ItemSize {
            width: width,
            height: height,
        }
    }
    fn to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

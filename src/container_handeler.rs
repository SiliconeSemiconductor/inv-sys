use std::string;

#[allow(dead_code)]
pub struct Container {
    name: String,
    max_weight: Option<f32>,
    widht: u32,
    height: u32,
    slots: Vec<u8>, // Stores filled slots in bits, for fast sorting or something
}

impl Container {
    pub fn build_container(
        name: String,
        max_weight: Option<f32>,
        widht: u32,
        height: u32,
    ) -> Container {
        Container {
            name: name,
            max_weight: max_weight,
            widht: widht,
            height: height,
            slots: build_container_slots(widht, height),
        }
    }
    pub fn print_container(&self) {
        println!(
            "name: {}\nmax_weight: {}\nwidth: {}\nheight: {} slots:\n{}",
            self.name,
            weight_to_string(self),
            self.widht,
            self.height,
            slots_to_string(self),
        );
    }
    pub fn print_container_fromated(&self) {
        for i in 0..self.widht {
            let row: String = String::new();
        }
    }
}

fn build_container_slots(widht: u32, height: u32) -> Vec<u8> {
    let slots: u32 = widht * height;
    let leftover: u32 = slots % 8;
    let slots: u32 = slots - leftover;
    let vector_item_count: u32 = slots / 8;
    let mut return_vector: Vec<u8> = Vec::new();

    for _i in 0..=vector_item_count {
        return_vector.push(0b00000000);
    }
    return return_vector;
}

fn weight_to_string(_self: &Container) -> String {
    match _self.max_weight {
        Some(float) => float.to_string(),
        None => String::from("None"),
    }
}
fn slots_to_string(_self: &Container) -> String {
    let mut returnstring = String::new();
    for i in &_self.slots {
        let token: String = format!("{:08b}\n", i);
        returnstring.push_str(token.as_str());
    }
    return returnstring;
}

// bit mask functions:
pub fn get_bit(mask: &Vec<u8>, index: usize) -> bool {
    let byte_index: usize = index / 8;
    let bit_index: usize = index % 8;

    (mask[byte_index] >> bit_index) & 1 == 1
}

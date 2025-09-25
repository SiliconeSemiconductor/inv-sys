mod item_handeler;
use item_handeler::Item;
use item_handeler::ItemType;

fn main() {
    let mut items: Vec<Item> = Vec::new();

    #[rustfmt::skip]items.push(Item::build_item("Jumpable Plate Carrier",0.6,"The JUMPABLE PLATE CARRIER™ (JPC) is a lightweight and minimal vest designed for maximum mobility, weight savings, and packability. At just over one pound for the entire carrier, the JPC™ offers a variety of configuration options to suit an operator’s needs in terms of load carriage and comfort. It features our SKELETAL™ CUMMERBUND system with our patented integrated attachment system that allows pouches to be mounted on both the inside and outside of the cummerbund, shedding unnecessary weight and bulk while improving ventilation. It also features integrated admin and magazine pouches on the front of the carrier. Sizes are based on the size of the ballistic plate. For example, a large plate will fit best in a large JPC™. Patented, see www.lwpatents.com. Made in US from US materials. ",ItemType::Armor, 2, 2));
    #[rustfmt::skip]items.push(Item::build_item("AK-105", 7.5,"Compact assault rifle packed with mods, optics, and extended mags; heavy but deadly in close to mid-range combat.", ItemType::Weapon, 3, 2));
    #[rustfmt::skip]items.push(Item::build_item("Iron Bar",20.0,"Made of iron.",ItemType::Recource,1, 1));
    #[rustfmt::skip]items.push(Item::build_item("Empty Bottle",0.25,"Empty bottle. No more beer :(",ItemType::Junk,1, 2));

    for i in items {
        i.print_item();
        i.print_item_extended();
    }
}

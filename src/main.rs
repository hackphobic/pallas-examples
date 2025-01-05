pub mod address;
use address::generate_address::xprv_from_phrase;

fn main() {
    let xprv = xprv_from_phrase("safe brief cactus coin solid zone minor east beef sock myth hair nose dry audit girl timber umbrella cloud table width ball cancel mango").expect("Error");
    print!("{:?}", xprv);
}
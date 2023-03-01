#![crate_type = "cdylib"]

extern crate rand;

use rand::Rng;

#[no_mangle]
pub extern "C" fn generate_color() -> String {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    format!("rgb({}, {}, {})", r, g, b)
}

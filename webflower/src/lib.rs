/* A library that returns Actix random flower */

use rand::Rng;

// create a const array of 5 flowers
const FLOWERS: [&str; 5] = ["Rose", "CherryBlossom", "Daisy", "Lotus", "Sunflower"];

// create a function that returns random flower
pub fn random_flower() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FLOWERS.len());
    FLOWERS[random_index].to_string()
}

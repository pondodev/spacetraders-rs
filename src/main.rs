pub mod api;
pub mod api_types;

use crate::api::API;

fn main() {
    let api = API::new(true);
    let factions = api.factions().unwrap();
    println!("got factions: {:?}", factions);
}


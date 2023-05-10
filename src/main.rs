pub mod api;
pub mod api_types;

use crate::api::API;
use tokio::task;

#[tokio::main]
async fn main() {
    let api = API::new(true);
    let task_handle = task::spawn(async move {
        match api.list_factions().await {
            Ok(f)  => println!("got factions: {:?}", f),
            Err(e) => println!("{}", e),
        }

        match api.get_faction("foo").await {
            Ok(f)  => println!("got faction: {:?}", f),
            Err(e) => println!("{}", e),
        }
    });

    tokio::join!(task_handle).0.unwrap();
}


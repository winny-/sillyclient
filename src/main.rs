extern crate tokio;

use std::env;
use sillypaste_cli_api::sillyrest::{*};

#[tokio::main]
async fn main() {
    let (username, password) =
        match (env::var("SILLY_USER"), env::var("SILLY_PASS")) {
            (Ok(u), Ok(p)) => (u, p),
            (_, _) => panic!("Be sure to set both SILLY_USER and SILLY_PASS in the environment."),
        };
    let cl = SillyPasteClient::new(
        username,
        password,
        "https://sillypaste.herokuapp.com".to_string()
    ).await.unwrap();
    let codes = cl.retrieve_language_codes().await;
    for (k, v) in codes {
        println!("{} -> {}", k, v);
    }
}


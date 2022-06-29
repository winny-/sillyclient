#[macro_use]
extern crate clap;
extern crate tokio;

use std::env;
use sillypaste_cli_api::sillyrest::{*};

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = clap::App::from_yaml(yaml);
    match app.clone().get_matches().subcommand() {
        Some(("list", _)) => {
            println!("listing ... (not implemented)");
        },
        Some(("paste", _)) => {
            println!("pasting ... (not implemented)");
        },
        Some(("languages", _)) => {
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
        },
        None | Some((_, _)) => {
            _ = app.print_help();  // Don't care if it works or no we're quitting.
            ()
        }
    };
    
}


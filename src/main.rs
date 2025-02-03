use tribesim::api::api_server::*;
use tribesim::cli::args::*;
use tribesim::config::file::*;
use tribesim::runtime::run_sim::*;
use tribesim::utils::*;

use clap::Parser;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if !args.launch_server {
        let config_path = env::var("SIM_CONFIG").expect("SIM_CONFIG must be set");
        match load_config_from_json(config_path.as_str()) {
            Ok(c) => {
                initiate_run(generate_uuid(), c).await;
            }
            Err(e) => {
                println!("Error loading config from file: {:?}", e);
            }
        }
    } else {
        let rocket = start_api_server();
        match rocket.launch().await {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to start web server: {}", e);
            }
        }
    }
}

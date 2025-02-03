extern crate rocket;
use crate::config::config::*;
use crate::config::file::*;
use crate::runtime::run_sim::*;

use crate::api::model::*;

use crate::utils::*;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, launch, routes};
use std::env;
use tokio::task;

fn spawn_sim(run_uuid: String, sim_config: SimConfig) {
    task::spawn(async move { initiate_run(run_uuid.clone(), sim_config).await });
}

#[get("/run_default_sim")]
async fn run_default_sim() -> Result<Json<RunSimResponse>, Status> {
    let config_path = env::var("SIM_CONFIG").expect("SIM_CONFIG must be set");
    let boilerplate_cfg =
        load_config_from_json(config_path.as_str()).map_err(|_| Status::new(500))?;
    let run_uuid = generate_uuid();

    spawn_sim(run_uuid.clone(), boilerplate_cfg);

    let response = RunSimResponse {
        status: format!("ok"),
        error: ResponseError::ok(),
        sim_uuid: run_uuid.clone(),
    };

    Ok(Json(response))
}

#[launch]
pub fn start_api_server() -> _ {
    rocket::build().mount("/api/v1/", routes![run_default_sim])
}

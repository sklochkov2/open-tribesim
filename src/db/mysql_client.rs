use crate::config::config::*;
use mysql_async::{prelude::*, Pool};
use serde_json;

/// Insert a run into the simulation_runs table, storing the config in JSON column.
pub async fn store_simulation_config(
    pool: &Pool,        // a connection pool to MySQL
    run_uuid: &str,     // unique identifier for this run
    config: &SimConfig, // your simulation config struct
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Serialize the struct to JSON
    let config_json = serde_json::to_string(config)?;

    // 2. Get a connection from the pool
    let mut conn = pool.get_conn().await?;

    // 3. Prepare the statement
    //    We'll insert a new row with run_uuid and JSON config.
    /*let insert_stmt = r#"
        INSERT INTO simulation_runs (run_uuid, config)
        VALUES (:run_uuid, JSON_OBJECT())
        ON DUPLICATE KEY UPDATE config = :config
    "#;*/

    // But MySQL doesn't let you do JSON_OBJECT() without arguments. We actually
    // want to insert a parameter. So let's keep it simpler:
    let insert_stmt = r#"
        INSERT INTO simulation_runs (run_uuid, config)
        VALUES (:run_uuid, :config_json)
    "#;

    // 4. Execute
    conn.exec_drop(
        insert_stmt,
        params! {
            "run_uuid" => run_uuid,
            "config_json" => config_json,
        },
    )
    .await?;

    // 5. Done
    Ok(())
}

pub async fn load_simulation_config(
    pool: &Pool,
    run_uuid: &str,
) -> Result<SimConfig, Box<dyn std::error::Error>> {
    let mut conn = pool.get_conn().await?;

    let row_opt: Option<(String,)> = conn
        .exec_first(
            "SELECT config FROM simulation_runs WHERE run_uuid = :run_uuid",
            params! {
                "run_uuid" => run_uuid,
            },
        )
        .await?;

    if let Some((config_json,)) = row_opt {
        let sim_config: SimConfig = serde_json::from_str(&config_json)?;
        Ok(sim_config)
    } else {
        Err(format!("No config found for run_uuid = {}", run_uuid).into())
    }
}

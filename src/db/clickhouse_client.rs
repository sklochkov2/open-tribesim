use clickhouse::{Client, Row};
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Matches the simulation_yearly_global_stats table
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct GlobalStatsRow {
    pub simulation_id: String,
    pub year: u32,

    pub total_memes_known: u64,
    pub avg_memes_known: f64,
    pub avg_trick_efficiency: f64,
    pub avg_brain_volume: f64,
    pub avg_meme_size: f64,
    // event_time has DEFAULT now(), so we omit it unless we want to supply it
}

/// Matches the simulation_yearly_meme_stats table
#[derive(Debug, Row, Serialize, Deserialize)]
pub struct MemeStatsRow {
    pub simulation_id: String,
    pub year: u32,
    pub meme_kind: String,

    pub avg_meme_efficiency: f64,
    pub avg_meme_size: f64,
    // event_time has DEFAULT now()
}

#[derive(Debug, Clone)]
pub struct DBCreds  {
    pub user: String,
    pub password: String,
    pub database: String,
}

pub async fn insert_global_stats(
    clickhouse_url: &str, creds: &DBCreds,
    rows: &[GlobalStatsRow],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a ClickHouse client
    let client = Client::default().with_url(clickhouse_url).with_user(&creds.user).with_password(&creds.password).with_database(&creds.database);

    // Build an inserter for the "simulation_yearly_global_stats" table
    let mut inserter = client
        .inserter("simulation_yearly_global_stats")?.with_timeouts(Some(Duration::from_secs(5)), Some(Duration::from_secs(20)))
    .with_max_bytes(50_000_000)
    .with_max_rows(750_000)
    .with_period(Some(Duration::from_secs(15)));  // returns InserterBuilder

    // Write each row to the inserter
    for row in rows {
        inserter.write(row)?;
    }

    // Commit the batch
    inserter.commit().await?;
    inserter.end().await?;

    Ok(())
}

pub async fn insert_meme_stats(
    clickhouse_url: &str, creds: &DBCreds,
    rows: &[MemeStatsRow],
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a ClickHouse client
    let client = Client::default().with_url(clickhouse_url).with_user(&creds.user).with_password(&creds.password).with_database(&creds.database);

    // Build an inserter for the "simulation_yearly_meme_stats" table
    let mut inserter = client
        .inserter("simulation_yearly_meme_stats")?.with_timeouts(Some(Duration::from_secs(5)), Some(Duration::from_secs(20)))
    .with_max_bytes(50_000_000)
    .with_max_rows(750_000)
    .with_period(Some(Duration::from_secs(15)));

    for row in rows {
        inserter.write(row)?;
    }

    inserter.commit().await?;
    inserter.end().await?;

    Ok(())
}

/*pub async fn insert_simulation_stats(
    clickhouse_url: &str,
    rows: &[SimulationYearlyStat],
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create client
    let client = Client::default()
        .with_url(clickhouse_url);

    // 2. Build an inserter for "simulation_yearly_stats"
    //    If the table is in a different database, call .with_database("...")
    let mut inserter = client
        .inserter("simulation_yearly_stats")? // This returns an InserterBuilder
//        .build()                               // returns Inserter in async
        .with_period(Some(Duration::from_secs(15)));

    // 3. Write each row
    for row in rows {
        inserter.write(row)?;
    }

    // 4. Commit the insert batch
    inserter.commit().await?;

    Ok(())
}*/

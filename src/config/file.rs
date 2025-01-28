use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::config::config::*;

pub fn load_config_from_json(path: &str) -> std::io::Result<SimConfig> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: SimConfig = serde_json::from_reader(reader)?;
    Ok(config)
}

pub fn save_config_to_json(path: &str, config: &SimConfig) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    Ok(())
}

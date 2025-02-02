use tribesim::simulation::group::*;

use tribesim::model::culture::*;
use tribesim::model::distribution::*;
use tribesim::model::population::*;
use tribesim::model::reproduction::*;

use tribesim::db::clickhouse_client::*;
use tribesim::db::mysql_client::*;

use tribesim::config::config::*;
use tribesim::config::file::*;

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use mysql_async::Pool;
use std::env;
use tokio;
use uuid::Uuid;
//use rand::prelude::*;

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

async fn multi_group_run(cfg: SimConfig, clickhouse_url: &str, mysql_pool: &Pool, creds: &DBCreds) {
    let mut rng = Xoshiro256PlusPlus::from_entropy();
    let resources: f64 = cfg.resources;
    let epoch = cfg.epoch;

    let init_members: usize = (cfg.resources as usize) / 17 / 3;
    let mut groups: Vec<Group> = vec![
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
    ];
    let sim_uuid: String = generate_uuid();

    match store_simulation_config(mysql_pool, sim_uuid.as_str(), &cfg).await {
        Ok(_) => {
            println!("Successfully stored configuration for run {}", sim_uuid);
        }
        Err(e) => {
            println!(
                "Failed to store configuration for run {} in mysql: {}",
                sim_uuid, e
            );
        }
    }

    let mut global_stats_batch: Vec<GlobalStatsRow> = Vec::new();
    let mut meme_stats_batch: Vec<MemeStatsRow> = Vec::new();

    for year in 0..epoch {
        groups.iter_mut().for_each(|group| {
            dinner_time(group);
            inventions(group, &cfg.meme_config, &mut rng);
            amnesia(group, &mut rng);
            perform_cultural_transfer(group, &mut rng, TransferMode::Teaching);
        });

        share_resources_across_groups(&mut groups, resources);

        groups.iter_mut().for_each(|group| {
            useless(group, &mut rng);
            perform_cultural_transfer(group, &mut rng, TransferMode::Learning);
            grim_reaper(group, &mut rng);
        });

        clean_up_groups(&mut groups);

        groups.iter_mut().for_each(|group| {
            reproduce_group(group, &mut rng, cfg.mutation_config);
        });

        handle_group_splitting(&mut groups, &mut rng);
        perform_migration(&mut groups, &mut rng, 0.001);

        global_stats_batch.push(build_general_statistics(sim_uuid.clone(), year, &groups));
        meme_stats_batch.extend(build_meme_statistics(sim_uuid.clone(), year, &groups));

        if (year + 1) % 1000 == 0 {
            match insert_global_stats(clickhouse_url, creds, &global_stats_batch).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while inserting into Clickhouse: {:?}", e);
                }
            }
            global_stats_batch = Vec::new();
            match insert_meme_stats(clickhouse_url, creds, &meme_stats_batch).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while inserting into Clickhouse: {:?}", e);
                }
            }
            meme_stats_batch = Vec::new();
        }

        groups.iter_mut().for_each(|group| {
            happy_new_year(group);
        });
        if (year + 1) % 10000 == 0 {
            println!("======== Group statistics for year {} ========", year);
            print_group_statistics(&groups);
            println!("==================================================");
        }
        if groups.len() == 0 {
            println!("Extinction at year {}!", year);
            break;
        }
    }
    match insert_global_stats(clickhouse_url, creds, &global_stats_batch).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error while inserting into Clickhouse: {:?}", e);
        }
    }
    match insert_meme_stats(clickhouse_url, creds, &meme_stats_batch).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error while inserting into Clickhouse: {:?}", e);
        }
    }
    print_group_statistics(&groups);
}

fn build_general_statistics(
    simulation_id: String,
    year: usize,
    groups: &Vec<Group>,
) -> GlobalStatsRow {
    let mut total_memes_known: u64 = 0;
    let mut headcount: u64 = 0;
    let mut avg_memes_known: f64 = 0.0;
    let mut avg_trick_efficiency: f64 = 0.0;
    let mut avg_brain_volume: f64 = 0.0;
    let mut avg_meme_size: f64 = 0.0;
    for group in groups {
        headcount += group.members.len() as u64;
        for member in &group.members {
            avg_memes_known += member.memes.len() as f64;
            total_memes_known += member.memes.len() as u64;
            avg_trick_efficiency += member.trick_efficiency;
            avg_brain_volume += member.get_brain_volume();
            for meme in &member.memes {
                avg_meme_size += meme.size;
            }
        }
    }
    if headcount < 1 {
        return GlobalStatsRow {
            simulation_id: simulation_id.clone(),
            year: year as u32,
            total_memes_known: 0,
            avg_memes_known: 0.0,
            avg_trick_efficiency: 0.0,
            avg_brain_volume: 0.0,
            avg_meme_size: 0.0,
        };
    }
    avg_memes_known = (total_memes_known as f64) / (headcount as f64);
    avg_trick_efficiency /= headcount as f64;
    avg_brain_volume /= headcount as f64;
    if total_memes_known > 0 {
        avg_meme_size /= total_memes_known as f64;
    }

    GlobalStatsRow {
        simulation_id: simulation_id.clone(),
        year: year as u32,
        total_memes_known: total_memes_known,
        avg_memes_known: avg_memes_known,
        avg_trick_efficiency: avg_trick_efficiency,
        avg_brain_volume: avg_brain_volume,
        avg_meme_size: avg_meme_size,
    }
}

fn build_meme_statistics(
    simulation_id: String,
    year: usize,
    groups: &Vec<Group>,
) -> Vec<MemeStatsRow> {
    let meme_types: Vec<MemeType> = vec![
        MemeType::Hunting,
        MemeType::Learning,
        MemeType::Teaching,
        MemeType::Trick,
        MemeType::Useless,
    ];
    let mut res: Vec<MemeStatsRow> = Vec::new();
    for meme_type in meme_types {
        let mut tot_memes: u64 = 0;
        let mut avg_size: f64 = 0.0;
        let mut avg_eff: f64 = 0.0;
        for group in groups {
            if group.members.len() == 0 {
                continue;
            }
            for member in &group.members {
                for meme in &member.memes {
                    if meme.kind == meme_type {
                        tot_memes += 1;
                        avg_size += meme.size;
                        avg_eff += meme.effect;
                    }
                }
            }
        }
        if tot_memes > 0 {
            avg_size /= tot_memes as f64;
            avg_eff /= tot_memes as f64;
        }
        res.push(MemeStatsRow {
            simulation_id: simulation_id.clone(),
            year: year as u32,
            meme_kind: format!("{:?}", meme_type),
            avg_meme_efficiency: avg_eff,
            avg_meme_size: avg_size,
        });
    }

    res
}

fn print_group_statistics(groups: &Vec<Group>) {
    for group in groups {
        println!(
            "Group {} ({} members) statistics",
            group.id,
            group.members.len()
        );
        let mut total_memes_known: usize = 0;
        let mut avg_meme_size: f64 = 0.0;
        let mut avg_mc: f64 = 0.0;
        let mut avg_hunting: f64 = 0.0;
        let mut avg_learning: f64 = 0.0;
        let mut avg_teaching: f64 = 0.0;
        let mut avg_trick: f64 = 0.0;
        let mut avg_useless: f64 = 0.0;
        for member in &group.members {
            total_memes_known += member.memes.len();
            avg_mc += member.mc_alleles.phenotype();
            avg_hunting += member.tot_hunting_efficiency;
            avg_learning += member.tot_learning_efficiency;
            avg_teaching += member.tot_teaching_efficiency;
            avg_trick += member.trick_efficiency;
            avg_useless += member.useless_probability;
            for meme in &member.memes {
                avg_meme_size += meme.size;
            }
        }
        if total_memes_known > 0 {
            avg_meme_size /= total_memes_known as f64;
        }
        let group_cnt: f64 = group.members.len() as f64;
        avg_mc /= group_cnt;
        avg_hunting /= group_cnt;
        avg_learning /= group_cnt;
        avg_teaching /= group_cnt;
        avg_trick /= group_cnt;
        avg_useless /= group_cnt;
        println!("Memes known: {}, avg size: {}, av_mc: {}, av_hu: {}, av_le: {}, av_te: {}, av_tre: {}, av_us: {}", total_memes_known, avg_meme_size, avg_mc, avg_hunting, avg_learning, avg_teaching, avg_trick, avg_useless);
    }
}

#[tokio::main]
async fn main() {
    let clickhouse_url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");
    let user = env::var("CLICKHOUSE_USER").expect("CLICKHOUSE_USER must be set");
    let password = env::var("CLICKHOUSE_PASSWORD").expect("CLICKHOUSE_PASSWORD must be set");
    let database = env::var("CLICKHOUSE_DB").expect("CLICKHOUSE_DB must be set");
    let mysql_url = env::var("MYSQL_URL").expect("MYSQL_URL must be set");
    let config_path = env::var("SIM_CONFIG").expect("SIM_CONFIG must be set");
    let sim_cfg: SimConfig;
    let pool = mysql_async::Pool::new(mysql_url.as_str());

    //match save_config_to_json("./cfg.json", &sim_cfg) {
    match load_config_from_json(config_path.as_str()) {
        Ok(c) => {
            sim_cfg = c;
            multi_group_run(
                sim_cfg,
                clickhouse_url.as_str(),
                &pool,
                &DBCreds {
                    user: user,
                    password: password,
                    database: database,
                },
            )
            .await;
        }
        Err(e) => {
            println!("Error loading config from file: {:?}", e);
        }
    }
}

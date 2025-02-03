use crate::simulation::group::*;

use crate::model::culture::*;
use crate::model::distribution::*;
use crate::model::population::*;
use crate::model::reproduction::*;

use crate::db::clickhouse_client::*;
use crate::db::mysql_client::*;

use crate::config::config::*;

use crate::runtime::statistics::*;

use mysql_async::Pool;

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use std::env;

pub async fn initiate_run(sim_uuid: String, cfg: SimConfig) {
    let clickhouse_url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");
    let user = env::var("CLICKHOUSE_USER").expect("CLICKHOUSE_USER must be set");
    let password = env::var("CLICKHOUSE_PASSWORD").expect("CLICKHOUSE_PASSWORD must be set");
    let database = env::var("CLICKHOUSE_DB").expect("CLICKHOUSE_DB must be set");
    let mysql_url = env::var("MYSQL_URL").expect("MYSQL_URL must be set");
    let pool = mysql_async::Pool::new(mysql_url.as_str());
    multi_group_run(
        sim_uuid,
        cfg,
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

pub async fn multi_group_run(
    sim_uuid: String,
    cfg: SimConfig,
    clickhouse_url: &str,
    mysql_pool: &Pool,
    creds: &DBCreds,
) {
    let mut rng = Xoshiro256PlusPlus::from_entropy();
    let resources: f64 = cfg.resources;
    let epoch = cfg.epoch;

    let init_members: usize = (cfg.resources as usize) / 17 / 3;
    let mut groups: Vec<Group> = vec![
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
        Group::new(init_members, cfg.agent_config, cfg.group_config, &mut rng),
    ];

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

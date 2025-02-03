use crate::config::config::*;
use crate::db::clickhouse_client::*;
use crate::simulation::group::*;

pub fn build_general_statistics(
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

pub fn build_meme_statistics(
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

pub fn print_group_statistics(groups: &Vec<Group>) {
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

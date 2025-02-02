use rand::Rng;
//use rand_distr::{Distribution, Normal};
//use std::collections::HashSet;

use crate::simulation::agent::*;
use crate::simulation::group::*;
use crate::simulation::memetics::*;

use crate::config::config::*;

#[derive(Debug, Clone, Copy)]
pub enum TransferMode {
    Learning,
    Teaching,
}

fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j, "Indices must differ");
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = slice.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}

pub fn inventions<R: Rng + ?Sized>(group: &mut Group, meme_cfg: &Vec<MemeConfig>, rng: &mut R) {
    group.members.iter_mut().for_each(|agent| {
        for config in meme_cfg {
            let roll = rng.gen::<f64>();
            if roll <= config.probability {
                let meme = Meme::new_typed(
                    config.meme_kind,
                    config.size.min,
                    config.size.max,
                    config.effect.min,
                    config.effect.max,
                    config.size_effect_corr,
                    rng,
                );
                agent.try_learning(meme);
                break;
            }
        }
    });
}

pub fn amnesia<R: Rng + ?Sized>(group: &mut Group, rng: &mut R) {
    group.members.iter_mut().for_each(|agent| {
        agent.memes.retain(|meme| {
            let roll = rng.gen::<f64>();
            if roll <= 0.02 {
                match meme.kind {
                    MemeType::Hunting => {
                        agent.tot_hunting_efficiency -= meme.effect;
                    }
                    MemeType::Learning => {
                        agent.tot_learning_efficiency -= meme.effect;
                    }
                    MemeType::Teaching => {
                        agent.tot_teaching_efficiency -= meme.effect;
                    }
                    MemeType::Trick => {
                        agent.trick_efficiency -= meme.effect;
                    }
                    MemeType::Useless => {
                        agent.useless_probability -= meme.effect;
                        if agent.useless_probability < 0.0 {
                            agent.useless_probability = 0.0;
                        }
                    }
                }
                agent.memory_used -= meme.size;
                agent.meme_id_set.remove(&meme.id);
                // TODO: decrease counter in meme library & check if the meme disappears forever
                //println!("Agent {} forgot meme {} of {:?} kind", agent.id, meme.id, meme.kind);
                return false;
            }
            return true;
        });
    });
}

pub fn useless<R: Rng + ?Sized>(group: &mut Group, rng: &mut R) {
    group.members.iter_mut().for_each(|agent| {
        if agent.useless_probability > 0.0 && agent.resources >= 1.0 {
            let roll = rng.gen::<f64>();
            if roll <= agent.useless_probability {
                agent.resources -= 1.0;
            }
        }
    });
}

pub fn perform_cultural_transfer<R: Rng + ?Sized>(
    group: &mut Group,
    rng: &mut R,
    mode: TransferMode,
) {
    let n = group.members.len();
    if n < 2 {
        // No interactions possible if there's < 2 members
        return;
    }

    for initiator_idx in 0..n {
        // Pick a random partner
        let partner_idx = rng.gen_range(0..n);
        if partner_idx == initiator_idx {
            // skip if same index to avoid dealing with self-learning/teaching
            continue;
        }

        // Borrow initiator & partner safely
        let (initiator, partner) = get_two_mut(&mut group.members, initiator_idx, partner_idx);

        match mode {
            TransferMode::Learning => {
                // The initiator tries to LEARN from the partner
                // => Meme must exist in `partner` but not in `initiator`
                cultural_exchange(
                    initiator, // "learner"
                    partner,   // "teacher"
                    rng, mode,
                );
            }
            TransferMode::Teaching => {
                // The initiator tries to TEACH the partner
                // => Meme must exist in `initiator` but not in `partner`
                cultural_exchange(
                    initiator, // "teacher"
                    partner,   // "student"
                    rng, mode,
                );
            }
        }
    }
}

/// A helper that, given two agents, picks an appropriate meme and tries to transfer it.
///
/// In "learning" mode, `agent_a` is the learner, `agent_b` is the teacher.
/// In "teaching" mode, `agent_a` is the teacher, `agent_b` is the student.
fn cultural_exchange<R: Rng + ?Sized>(
    agent_a: &mut Agent,
    agent_b: &mut Agent,
    rng: &mut R,
    mode: TransferMode,
) {
    // Identify which side is the "teacher" vs. "student" for this exchange
    let (teacher, student) = match mode {
        TransferMode::Learning => (agent_b, agent_a), // B->A
        TransferMode::Teaching => (agent_a, agent_b), // A->B
    };

    if teacher.memes.is_empty() {
        return;
    }

    let p_success = match mode {
        TransferMode::Learning => student.tot_learning_efficiency,
        TransferMode::Teaching => student.tot_learning_efficiency + teacher.tot_teaching_efficiency,
    };
    //let p_success:f64 = 0.1;

    if p_success == 0.0 {
        return;
    }

    // 1. Which memes does the teacher have that the student doesn't?
    //let student_known: HashSet<_> = student.culture.memes.iter().map(|m| m.id).collect();
    let student_known = &student.meme_id_set;

    // If there's no new meme to transfer, fail immediately
    let possible_memes = &teacher.memes;
    for _ in 0..3 {
        // 2. Randomly pick one meme from teacher's memes
        let chosen_meme = possible_memes[rng.gen_range(0..possible_memes.len())];
        // If the student doesn't know it, initiate transfer attempt
        if !student_known.contains(&chosen_meme.id) {
            if rng.gen::<f64>() <= p_success {
                let meme_clone = chosen_meme.clone();
                if student.try_learning(meme_clone) {
                    // TODO: update meme library
                    //println!("A meme of {:?} kind has been transferred", chosen_meme.kind);
                }
            }
            break;
        }
    }
}

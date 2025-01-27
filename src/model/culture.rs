use rand::Rng;
//use rand_distr::{Distribution, Normal};
//use std::collections::HashSet;

use crate::simulation::agent::*;
use crate::simulation::group::*;
use crate::simulation::memetics::*;

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

pub fn inventions<R: Rng + ?Sized>(group: &mut Group, rng: &mut R) {
    group.members.iter_mut().for_each(|agent| {
        let roll = rng.gen::<f64>();
        if roll <= 0.000532 {
            let meme = Meme::new_random(rng);
            if agent.try_learning(meme) {
                // Add meme to meme library - once it's implemented
                //println!("Agent {} invented meme {} of {:?} kind", agent.id, meme.id, meme.kind);
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
                        /*if agent.tot_learning_efficiency < 0.0 {
                            agent.tot_learning_efficiency = 0.0;
                        }*/
                    }
                    MemeType::Teaching => {
                        agent.tot_teaching_efficiency -= meme.effect;
                        /*if agent.tot_teaching_efficiency < 0.0 {
                            agent.tot_teaching_efficiency = 0.0;
                        }*/
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

/*pub fn perform_learning<R: Rng + ?Sized>(group: &mut Group, rng: &mut R) {
    let n = group.members.len();
    if n < 2 {
        // No learning possible if there's 0 or 1 member
        return;
    }

    for student_idx in 0..n {
        // 1. Pick a random teacher index
        //let mut teacher_idx = rng.gen_range(0..n);
        let teacher_idx: usize;
        loop {
            let roll = rng.gen_range(0..n);
            if roll != student_idx {
                teacher_idx = roll;
                break;
            }
        }
        

        // If we don't want teacher == student, skip or re-roll
        if teacher_idx == student_idx {
            // Easiest fix: skip if the chosen teacher is the same
            // Or re-roll with logic: teacher_idx = (teacher_idx + 1) % n;
            continue;
        }

        // 2. Safely borrow the student mutably and the teacher immutably
        let (student, teacher) = {
            // We only need a mutable reference to the "student" so we can
            // add memes to them. The teacher can be immutable. We'll do that
            // by calling `get_two_mut()` for *both* if you think you might
            // mutate teacher resources. If you only read from teacher,
            // you can do teacher as an immutable reference from the same slice.
            
            // We'll do the "two mut" approach in case we ever want to mutate teacher too.
            let (s, t) = get_two_mut(&mut group.members, student_idx, teacher_idx);
            (s, t as &Agent) // cast t to &Agent if you prefer not to mutate teacher
        };

        // 3. Gather memes unknown to the student
        let student_known: HashSet<_> = student.culture.memes.iter().map(|m| m.id).collect();
        let mut unknown_memes: Vec<_> = teacher
            .culture
            .memes
            .iter()
            .filter(|m| !student_known.contains(&m.id))
            .collect();

        // If none are unknown, learning attempt fails
        if unknown_memes.is_empty() {
            continue;
        }

        // 4. Pick one random meme
        let pick = rng.gen_range(0..unknown_memes.len());
        let meme = unknown_memes.swap_remove(pick);

        // 5. Attempt success with prob = student.learning_efficiency
        let p_success = student.learning_efficiency;
        if rng.gen::<f64>() < p_success {
            // Student acquires the meme
            // Ensure Meme is Clone (or we store references) so we can push a copy
            //student.culture.memes.push(meme.clone());
            if student.try_learning(meme.clone()) {
                println!("A meme of {:?} kind has been learnt!", meme.kind);
            }
        }
    }
}*/

pub fn perform_cultural_transfer<R: Rng + ?Sized>(group: &mut Group, rng: &mut R, mode: TransferMode) {
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
                    initiator,  // "learner"
                    partner,    // "teacher"
                    rng,
                    mode,
                );
            }
            TransferMode::Teaching => {
                // The initiator tries to TEACH the partner
                // => Meme must exist in `initiator` but not in `partner`
                cultural_exchange(
                    initiator,  // "teacher"
                    partner,    // "student"
                    rng,
                    mode,
                );
            }
        }
    }
}

/// A helper that, given two agents, picks an appropriate meme and tries to transfer it.
///
/// In "learning" mode, `agent_a` is the learner, `agent_b` is the teacher.
/// In "teaching" mode, `agent_a` is the teacher, `agent_b` is the student.
fn cultural_exchange<R: Rng + ?Sized>(agent_a: &mut Agent, agent_b: &mut Agent, rng: &mut R, mode: TransferMode) {
    // Identify which side is the "teacher" vs. "student" for this exchange
    let (teacher, student) = match mode {
        TransferMode::Learning => (agent_b, agent_a),  // B->A
        TransferMode::Teaching => (agent_a, agent_b),  // A->B
    };

    if teacher.memes.is_empty() {
        return;
    }

    let p_success = match mode {
        TransferMode::Learning => {
            student.tot_learning_efficiency
        }
        TransferMode::Teaching => {
            student.tot_learning_efficiency + teacher.tot_teaching_efficiency
        }
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


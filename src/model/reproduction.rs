use crate::simulation::agent::*;
use crate::simulation::group::*;

use crate::config::config::*;

use rand::prelude::*;
use rand::Rng;

pub fn attempt_reproduction<R: Rng + ?Sized>(
    parent_a: &mut Agent,
    parent_b: &mut Agent,
    mut_cfg: MutationCfg,
    rng: &mut R,
) -> Option<Agent> {
    // Step 1: Child inherits one allele from each parent for each trait.
    // memory_capacity:
    let child_mc_allele_from_a = if rng.gen_bool(0.5) {
        parent_a.mc_alleles.allele1
    } else {
        parent_a.mc_alleles.allele2
    };
    let child_mc_allele_from_b = if rng.gen_bool(0.5) {
        parent_b.mc_alleles.allele1
    } else {
        parent_b.mc_alleles.allele2
    };

    // learning_efficiency:
    let child_le_allele_from_a = if rng.gen_bool(0.5) {
        parent_a.le_alleles.allele1
    } else {
        parent_a.le_alleles.allele2
    };
    let child_le_allele_from_b = if rng.gen_bool(0.5) {
        parent_b.le_alleles.allele1
    } else {
        parent_b.le_alleles.allele2
    };

    // teaching_efficiency:
    let child_te_allele_from_a = if rng.gen_bool(0.5) {
        parent_a.te_alleles.allele1
    } else {
        parent_a.te_alleles.allele2
    };
    let child_te_allele_from_b = if rng.gen_bool(0.5) {
        parent_b.te_alleles.allele1
    } else {
        parent_b.te_alleles.allele2
    };

    // Step 2: Apply mutation *individually* to each allele
    //   We'll do it using your existing mutation approach, e.g.:
    let mc_mutation = mut_cfg.mem_mutation;
    let le_mutation = mut_cfg.learning_mutation;
    let te_mutation = mut_cfg.teaching_mutation;

    let mut child_mc_alleles = Alleles {
        allele1: mc_mutation.mutate_value(child_mc_allele_from_a, rng),
        allele2: mc_mutation.mutate_value(child_mc_allele_from_b, rng),
    };
    // clamp to >= 0
    if child_mc_alleles.allele1 < 0.0 {
        child_mc_alleles.allele1 = 0.0;
    }
    if child_mc_alleles.allele2 < 0.0 {
        child_mc_alleles.allele2 = 0.0;
    }

    let mut child_le_alleles = Alleles {
        allele1: le_mutation.mutate_value(child_le_allele_from_a, rng),
        allele2: le_mutation.mutate_value(child_le_allele_from_b, rng),
    };
    // clamp within [0.0, 1.0]
    for val in &mut [ &mut child_le_alleles.allele1, &mut child_le_alleles.allele2 ] {
        if **val < 0.0 { **val = 0.0; }
        if **val > 1.0 { **val = 1.0; }
    }

    let mut child_te_alleles = Alleles {
        allele1: te_mutation.mutate_value(child_te_allele_from_a, rng),
        allele2: te_mutation.mutate_value(child_te_allele_from_b, rng),
    };
    // clamp within [0.0, 1.0]
    for val in &mut [ &mut child_te_alleles.allele1, &mut child_te_alleles.allele2 ] {
        if **val < 0.0 { **val = 0.0; }
        if **val > 1.0 { **val = 1.0; }
    }

    // Step 3: Build a "potential child" to compute brain volume
    let potential_child = Agent::newborn(
        rng,
        child_mc_alleles,
        child_le_alleles,
        child_te_alleles,
        parent_a.config.clone(),
    );

    // Step 4: Reproduction cost = 2 * child's brain volume
    let child_brain_volume = potential_child.get_brain_volume();
    let child_cost = 2.0 * child_brain_volume;

    // Check resources from both parents
    let mut resource_pool = parent_a.resources + parent_b.resources;
    if resource_pool < child_cost {
        return None; // not enough
    }

    // Deduct cost, leftover is resource_pool
    resource_pool -= child_cost;

    // 40% -> child, 60% -> equally for parents
    let child_share = 0.40 * resource_pool;
    let parent_share_each = 0.30 * resource_pool;

    parent_a.resources = parent_share_each;
    parent_b.resources = parent_share_each;

    let mut child = potential_child;
    child.resources = child_share;

    Some(child)
}

pub fn reproduce_group<R: Rng + ?Sized>(group: &mut Group, rng: &mut R, mut_cfg: MutationCfg) {
    let mut eligible_indices: Vec<usize> = group
        .members
        .iter()
        .enumerate()
        .filter(|(_, agent)| agent.age >= 6)
        .map(|(i, _)| i)
        .collect();

    eligible_indices.shuffle(rng);

    let mut children = Vec::new();

    for pair in eligible_indices.chunks(2) {
        if pair.len() == 2 {
            let i1 = pair[0];
            let i2 = pair[1];

            let (i1, i2) = if i1 < i2 { (i1, i2) } else { (i2, i1) };

            let (left, right) = group.members.split_at_mut(i2);
            let parent_a = &mut left[i1];
            let parent_b = &mut right[0];

            if let Some(child) = attempt_reproduction(parent_a, parent_b, mut_cfg, rng) {
                children.push(child);
            }
        }
    }

    group.members.extend(children);
}

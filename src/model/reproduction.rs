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
    let mc_mutation = mut_cfg.mem_mutation;
    let le_mutation = mut_cfg.learning_mutation;
    let te_mutation = mut_cfg.teaching_mutation;
    // 1. Inherit memory_capacity, learning_efficiency, teaching_efficiency from parents
    let child_mc = if rng.gen_bool(0.5) {
        parent_a.memory_capacity
    } else {
        parent_b.memory_capacity
    };

    let child_le = if rng.gen_bool(0.5) {
        parent_a.learning_efficiency
    } else {
        parent_b.learning_efficiency
    };

    let child_te = if rng.gen_bool(0.5) {
        parent_a.teaching_efficiency
    } else {
        parent_b.teaching_efficiency
    };

    // 2. Apply mutations & clamp within valid ranges
    let mut mc_mutated = mc_mutation.mutate_value(child_mc, rng);
    if mc_mutated < 0.0 {
        mc_mutated = 0.0;
    }

    let mut le_mutated = le_mutation.mutate_value(child_le, rng);
    if le_mutated < 0.0 {
        le_mutated = 0.0;
    } else if le_mutated > 1.0 {
        le_mutated = 1.0;
    }

    let mut te_mutated = te_mutation.mutate_value(child_te, rng);
    if te_mutated < 0.0 {
        te_mutated = 0.0;
    } else if te_mutated > 1.0 {
        te_mutated = 1.0;
    }

    // pub fn newborn(rng: &mut StdRng, mc: f64, le: f64, te: f64, cfg: AgentCfg) -> Self
    // 3. Create a "potential child" to compute its brain_volume
    // The config is the same for everyone at this stage; if it changes, additional logic is
    // required
    let potential_child = Agent::newborn(
        rng,
        mc_mutated,
        le_mutated,
        te_mutated,
        parent_a.config.clone(),
    );

    // 4. Compute the child's brain volume -> cost each parent must pay
    // According to the step: "both parents need to spend 2 * child's brain volume"
    let child_brain_volume = potential_child.get_brain_volume();
    let child_cost = 2.0 * child_brain_volume;
    let mut resource_pool = parent_a.resources + parent_b.resources;

    // 5. Check if each parent has enough resources
    if resource_pool < child_cost {
        // Not enough resources -> fail
        return None;
    }

    // 6. Deduct cost
    resource_pool -= child_cost;

    // 7. Distribute leftover resources: child gets 40%, parents split 60%
    // 40% -> child's resource
    let child_share = 0.40 * resource_pool;
    // 60% -> equally split between the two parents
    let parent_share_each = 0.30 * resource_pool;

    parent_a.resources = parent_share_each;
    parent_b.resources = parent_share_each;

    // 8. Build the final child struct with the assigned resources
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

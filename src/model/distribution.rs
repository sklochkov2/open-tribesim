use crate::simulation::group::*;

pub fn distribute_resources(group: &mut Group, total_resources: f64) {
    // 1. Calculate the sum of (1 + trick_efficiency) for all agents
    let denominator: f64 = group
        .members
        .iter()
        .map(|agent| 1.0 + agent.trick_efficiency)
        .sum();

    // 2. Distribute resources proportionally
    for agent in group.members.iter_mut() {
        let share_ratio = (1.0 + agent.trick_efficiency) / denominator;
        agent.resources = share_ratio * total_resources;
    }
}

pub fn hunting_effort(group: &mut Group) -> f64 {
    group
        .members
        .iter_mut()
        .map(|agent| {
            if agent.resources >= 2.0 {
                agent.resources -= 2.0;
                agent.tot_hunting_efficiency
            } else {
                0.0
            }
        })
        .sum()
}

pub fn share_resources_across_groups(groups: &mut Vec<Group>, total_resources: f64) {
    // 1. Compute the hunting effort for each group
    let efforts: Vec<f64> = groups
        .iter_mut()
        .map(|group| hunting_effort(group))
        .collect();

    // 2. Calculate the sum of all efforts
    let total_effort: f64 = efforts.iter().sum();

    // 3. If total_effort is 0, no group gets anything
    if total_effort <= f64::EPSILON {
        for group in groups.iter_mut() {
            distribute_resources(group, 0.0);
        }
        return;
    }

    // 4. Otherwise, distribute resources proportionally
    for (i, group) in groups.iter_mut().enumerate() {
        let proportion = efforts[i] / total_effort;
        let group_share = proportion * total_resources;
        distribute_resources(group, group_share);
    }
}

pub fn clean_up_groups(groups: &mut Vec<Group>) {
    groups.retain(|group| {
        group.members.len() > 1
    });
}

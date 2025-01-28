use crate::simulation::group::*;
use rand::Rng;

pub fn dinner_time(group: &mut Group) {
    // TODO: make the appetites configurable.
    for agent in group.members.iter_mut() {
        if agent.resources >= 3.0 {
            agent.resources -= 3.0;
            agent.hunger_counter = 0;
        } else {
            agent.resources = 0.0;
            agent.hunger_counter += 1;
        }
    }
}

pub fn happy_new_year(group: &mut Group) {
    for agent in group.members.iter_mut() {
        agent.age += 1;
    }
}

pub fn grim_reaper<R: Rng + ?Sized>(group: &mut Group, rng: &mut R) -> usize {
    let before_count = group.members.len();
    group.members.retain(|agent| {
        if agent.hunger_counter >= 2 {
            return false;
        }

        let p_death = agent.config.death_prob_multiplier * (agent.age as f64);
        let roll = rng.gen::<f64>();
        if roll < p_death {
            return false;
        }

        true
    });
    let after_count = group.members.len();

    before_count - after_count
}

pub fn perform_migration<R: Rng + ?Sized>(
    groups: &mut Vec<Group>,
    rng: &mut R,
    migration_prob: f64,
) {
    let n = groups.len();

    // If there's only one group, there's nowhere to migrate
    // so we skip.
    if n < 2 {
        return;
    }

    // For each group in the vector...
    for i in 0..n {
        // We'll iterate the group's members from the front,
        // but each time an agent migrates, we `swap_remove`
        // them, so we stay at the same index to examine
        // the newly swapped-in member.
        let mut j = 0;
        while j < groups[i].members.len() {
            let roll = rng.gen::<f64>();
            if roll < migration_prob {
                // This agent migrates.
                // 1) Remove it from group i:
                let agent = groups[i].members.swap_remove(j);

                // 2) Choose a random group different from i
                let mut new_group_idx = rng.gen_range(0..n);
                while new_group_idx == i {
                    new_group_idx = rng.gen_range(0..n);
                }

                // 3) Push agent into the new group
                groups[new_group_idx].members.push(agent);

                // We do *not* increment j here, because swap_remove
                // has already pulled a new member into index j,
                // so we want to examine that one next iteration.
            } else {
                // This agent does not migrate, so move on
                j += 1;
            }
        }
    }
}

pub fn handle_group_splitting<R: Rng + ?Sized>(groups: &mut Vec<Group>, rng: &mut R) {
    // We'll build a new list of groups
    // that replaces the old list in-place when done.
    let mut new_groups = Vec::new();

    // Drain the existing groups so we can consume them one by one
    for mut group in groups.drain(..) {
        // If group is over capacity, split it
        if group.members.len() > group.config.max_size {
            /*let mut group_a = Group {
                id: new_id(rng),      // new ID
                members: Vec::new(),  // will distribute half the members here
                config: group.config, // same config
            };
            let mut group_b = Group {
                id: new_id(rng),
                members: Vec::new(),
                config: group.config,
            };*/
            // pub fn splinter<R: Rng + ?Sized>(members: &Vec<Agent>, group_cfg: GroupCfg, rng: &mut R)
            let mut group_a = Group::splinter(&Vec::new(), group.config, rng);
            let mut group_b = Group::splinter(&Vec::new(), group.config, rng);

            // Randomly distribute each member of the old group
            // with equal probability between the two new groups
            for agent in group.members.drain(..) {
                if rng.gen::<f64>() < 0.5 {
                    group_a.members.push(agent);
                } else {
                    group_b.members.push(agent);
                }
            }

            // Now we have two new subgroups
            new_groups.push(group_a);
            new_groups.push(group_b);
        } else {
            // If group is under or at capacity, keep it as-is
            new_groups.push(group);
        }
    }

    // Replace the old list of groups
    *groups = new_groups;
}

use crate::simulation::agent::*;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct GroupCfg {
    pub max_size: usize,
}

#[derive(Debug)]
pub struct Group {
    pub id: usize,
    pub members: Vec<Agent>,
    pub config: GroupCfg,
}

fn new_id<R: Rng + ?Sized>(rng: &mut R) -> usize {
    rng.gen::<usize>()
}

impl Group {
    pub fn new<R: Rng + ?Sized>(size: usize, member_cfg: AgentCfg, group_cfg: GroupCfg, rng: &mut R) -> Self {
        //let a1 = Agent::default(&mut rng, cfg);
        let mut members: Vec<Agent> = Vec::new();
        for _ in 0..size {
            members.push(Agent::default(rng, member_cfg));
        }
        Self {
            id: new_id(rng),
            members: members,
            config: group_cfg,
        }
    }

    pub fn splinter<R: Rng + ?Sized>(members: &Vec<Agent>, group_cfg: GroupCfg, rng: &mut R) -> Self {
        Self {
            id: new_id(rng),
            members: members.clone(),
            config: group_cfg,
        }
    }
}

use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::collections::HashSet;

use crate::simulation::memetics::*;

#[derive(Copy, Clone, Debug)]
pub struct AgentCfg {
    pub base_brain_volume: f64,
    pub mem_cost: f64,
    pub death_prob_multiplier: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MutationParams {
    pub probability: f64,
    pub magnitude_std: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct MutationCfg {
    pub mem_mutation: MutationParams,
    pub learning_mutation: MutationParams,
    pub teaching_mutation: MutationParams,
}

// Culture contains the set of memes an individual possesses along with
// the attribute modifiers which are recomputed every time a meme is learnt
// or forgotten in order to minimise the number of iterations through the memes.
/*#[derive(Debug, Clone)]
pub struct Culture {
    pub learning_efficiency: f64,
    pub teaching_efficiency: f64,
    pub hunting_efficiency: f64,
    pub trick_efficiency: f64,
    pub useless_probability: f64,
    pub size: f64,
    pub memes: Vec<Meme>,
    pub meme_id_set: HashSet<usize>,
}*/

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: usize,
    pub age: u8,
    pub hunger_counter: u8,
    pub memory_capacity: f64,
    pub learning_efficiency: f64,
    pub tot_learning_efficiency: f64,
    pub teaching_efficiency: f64,
    pub tot_teaching_efficiency: f64,
    pub hunting_efficiency: f64,
    pub tot_hunting_efficiency: f64,
    pub useless_probability: f64,
    pub trick_efficiency: f64,
    pub memory_used: f64,
    pub resources: f64,
    pub memes: Vec<Meme>,
    pub meme_id_set: HashSet<usize>,
    pub config: AgentCfg,
//    pub culture: Culture,
}

impl AgentCfg {
    pub fn default() -> Self {
        Self {
            base_brain_volume: 20.0,
            mem_cost: 1.0,
            death_prob_multiplier: 0.002,
        }
    }
}

impl MutationParams {
    pub fn mutate_value<R: Rng + ?Sized>(&self, current: f64, rng: &mut R) -> f64 {
        if rng.gen::<f64>() < self.probability {
            let normal = Normal::new(0.0, self.magnitude_std).expect("Invalid normal parameters");
            let change = normal.sample(rng);
            current + change
        } else {
            current
        }
    }
}

/*impl Culture {
    pub fn default() -> Self {
        Self {
            learning_efficiency: 0.0,
            teaching_efficiency: 0.0,
            hunting_efficiency: 0.0,
            trick_efficiency: 0.0,
            useless_probability: 0.0,
            size: 0.0,
            memes: Vec::new(),
            meme_id_set: HashSet::new(),
        }
    }
}*/

fn new_id<R: Rng + ?Sized>(rng: &mut R) -> usize {
    rng.gen::<usize>()
}

impl Agent {
    pub fn default<R: Rng + ?Sized>(rng: &mut R, cfg: AgentCfg) -> Self {
        Self {
            id: new_id(rng),
            age: 0,
            hunger_counter: 0,
            memory_capacity: 0.0,
            learning_efficiency: 0.1,
            tot_learning_efficiency: 0.1,
            teaching_efficiency: 0.0,
            tot_teaching_efficiency: 0.0,
            hunting_efficiency: 10.0,
            tot_hunting_efficiency: 10.0,
            trick_efficiency: 0.0,
            useless_probability: 0.0,
            memory_used: 0.0,
            resources: 10.0,
            memes: Vec::new(),
            meme_id_set: HashSet::new(),
            config: cfg,
//            culture: Culture::default(),
        }
    }

    pub fn newborn<R: Rng + ?Sized>(rng: &mut R, mc: f64, le: f64, te: f64, cfg: AgentCfg) -> Self {
        Self {
            id: new_id(rng),
            age: 0,
            hunger_counter: 0,
            memory_capacity: mc,
            learning_efficiency: le,
            tot_learning_efficiency: le,
            teaching_efficiency: te,
            tot_teaching_efficiency: te,
            hunting_efficiency: 10.0,
            tot_hunting_efficiency: 10.0,
            trick_efficiency: 0.0,
            useless_probability: 0.0,
            memory_used: 0.0,
            resources: 0.0,
            memes: Vec::new(),
            meme_id_set: HashSet::new(),
            config: cfg,
            //culture: Culture::default(),
        }
    }

    pub fn get_brain_volume(&self) -> f64 {
        self.config.base_brain_volume + self.config.mem_cost * self.memory_capacity
    }

    pub fn to_string(&self) -> String {
        format!(
            "id: {}, age: {}, h: {}, mc: {}, le: {}, te: {}, he: {}, tre: {}, up: {}, bv: {}, res: {}",
            self.id,
            self.age,
            self.hunger_counter,
            self.memory_capacity,
            self.tot_learning_efficiency,
            self.tot_teaching_efficiency,
            self.tot_hunting_efficiency,
            self.trick_efficiency,
            self.useless_probability,
            self.get_brain_volume(),
            self.resources
        )
    }

    pub fn try_learning(&mut self, m: Meme) -> bool {
        if self.memory_capacity - self.memory_used < m.size {
            return false;
        }
        self.memory_used += m.size;
        self.memes.push(m);
        self.meme_id_set.insert(m.id);
        match m.kind {
            MemeType::Hunting => {
                self.tot_hunting_efficiency += m.effect;
            }
            MemeType::Learning => {
                self.tot_learning_efficiency += m.effect;
                /*if self.tot_learning_efficiency > 1.0 {
                    self.tot_learning_efficiency = 1.0;
                }*/
            }
            MemeType::Teaching => {
                self.tot_teaching_efficiency += m.effect;
                /*if self.tot_teaching_efficiency > 1.0 {
                    self.tot_teaching_efficiency = 1.0;
                }*/
            }
            MemeType::Trick => {
                self.trick_efficiency += m.effect;
            }
            MemeType::Useless => {
                self.useless_probability += m.effect;
                /*if self.useless_probability > 1.0 {
                    self.useless_probability = 1.0;
                }*/
            }
        }
        true
    }
}

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MemeType {
    Hunting,
    Learning,
    Teaching,
    Trick,
    Useless,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AgentCfg {
    pub base_brain_volume: f64,
    pub mem_cost: f64,
    pub death_prob_multiplier: f64,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MutationParams {
    pub probability: f64,
    pub magnitude_std: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MutationCfg {
    pub mem_mutation: MutationParams,
    pub learning_mutation: MutationParams,
    pub teaching_mutation: MutationParams,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GroupCfg {
    pub max_size: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MemeConfig {
    pub meme_kind: MemeType,
    pub probability: f64,
    pub size: Range,
    pub effect: Range,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimConfig {
    pub mutation_config: MutationCfg,
    pub agent_config: AgentCfg,
    pub meme_config: Vec<MemeConfig>,
    pub group_config: GroupCfg,
    pub epoch: usize,
    pub resources: f64,
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

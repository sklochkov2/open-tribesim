use rand::Rng;
use std::collections::HashSet;

use crate::config::config::*;
use crate::simulation::memetics::*;

#[derive(Debug, Clone, Copy)]
pub struct Alleles {
    pub allele1: f64,
    pub allele2: f64,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: usize,
    pub age: u8,
    pub hunger_counter: u8,
    pub mc_alleles: Alleles,
    pub le_alleles: Alleles,
    pub te_alleles: Alleles,
    pub tot_learning_efficiency: f64,
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
}

fn new_id<R: Rng + ?Sized>(rng: &mut R) -> usize {
    rng.gen::<usize>()
}

impl Alleles {
    /// Returns the average (phenotypic expression).
    pub fn phenotype(&self) -> f64 {
        0.5 * (self.allele1 + self.allele2)
    }
}

impl Agent {
    pub fn default<R: Rng + ?Sized>(rng: &mut R, cfg: AgentCfg) -> Self {
        Self {
            id: new_id(rng),
            age: 0,
            hunger_counter: 0,
            mc_alleles: Alleles{allele1: 0.0, allele2: 0.0},
            le_alleles: Alleles{allele1: 0.1, allele2: 0.1},
            te_alleles: Alleles{allele1: 0.0, allele2: 0.0},
            tot_learning_efficiency: 0.1,
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
        }
    }

    pub fn newborn<R: Rng + ?Sized>(
        rng: &mut R,
        mc_alleles: Alleles,
        le_alleles: Alleles,
        te_alleles: Alleles,
        cfg: AgentCfg,
    ) -> Self {
        // We'll compute the phenotypes for the newly minted agent
        let le_phenotype = le_alleles.phenotype();
        let te_phenotype = te_alleles.phenotype();

        Self {
            id: new_id(rng),
            age: 0,
            hunger_counter: 0,
            mc_alleles,
            le_alleles,
            te_alleles,
            tot_learning_efficiency: le_phenotype,
            tot_teaching_efficiency: te_phenotype,
            tot_hunting_efficiency: 10.0,
            hunting_efficiency: 10.0,
            trick_efficiency: 0.0,
            useless_probability: 0.0,
            memory_used: 0.0,
            resources: 0.0,
            memes: Vec::new(),
            meme_id_set: HashSet::new(),
            config: cfg,
        }
    }

    pub fn get_brain_volume(&self) -> f64 {
        self.config.base_brain_volume + self.config.mem_cost * self.mc_alleles.phenotype()
    }

    pub fn to_string(&self) -> String {
        format!(
            "id: {}, age: {}, h: {}, mc: {}, le: {}, te: {}, he: {}, tre: {}, up: {}, bv: {}, res: {}",
            self.id,
            self.age,
            self.hunger_counter,
            self.mc_alleles.phenotype(),
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
        if self.mc_alleles.phenotype() - self.memory_used < m.size {
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
            }
            MemeType::Teaching => {
                self.tot_teaching_efficiency += m.effect;
            }
            MemeType::Trick => {
                self.trick_efficiency += m.effect;
            }
            MemeType::Useless => {
                self.useless_probability += m.effect;
            }
        }
        true
    }
}

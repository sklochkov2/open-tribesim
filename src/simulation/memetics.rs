use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MemeType {
    Hunting,
    Learning,
    Teaching,
    Trick,
    Useless,
}

#[derive(Debug, Clone, Copy)]
pub struct Meme {
    pub id: usize,
    pub size: f64,
    pub kind: MemeType,
    pub effect: f64,
}

fn new_id<R: Rng + ?Sized>(rng: &mut R) -> usize {
    rng.gen::<usize>()
}

impl Meme {
    /// Creates a new `Meme` with random type, effect, and size.
    ///
    /// - `id`: Unique ID for the meme.
    /// - `min_size`, `max_size`: Range for meme size; actual size is
    ///   sampled from a normal distribution centered at the midpoint and
    ///   clamped to [min_size, max_size].
    /// - `rng`: A mutable reference to any RNG implementing `rand::Rng`.
    pub fn new_random<R: Rng + ?Sized>(
        rng: &mut R,
    ) -> Meme {
        // TODO: pack all arbitrary values into a configuration struct
        // 1. Pick a random MemeType
        let kind_index = rng.gen_range(0..=4);
        let kind = match kind_index {
            0 => MemeType::Hunting,
            1 => MemeType::Learning,
            2 => MemeType::Teaching,
            3 => MemeType::Trick,
            _ => MemeType::Useless,
        };
        let min_size: f64 = 0.3;
        let max_size: f64 = 0.8;

        // 2. Assign an effect range per MemeType (example ranges)
        let (effect_min, effect_max) = match kind {
            MemeType::Hunting => (1.3, 6.2),
            MemeType::Learning => (0.3, 0.8),
            MemeType::Teaching => (0.09, 0.6),
            MemeType::Trick => (0.9, 2.8),
            MemeType::Useless => (0.1, 0.4),
        };
        // Generate effect within that range
        let effect = rng.gen_range(effect_min..effect_max);

        // 3. Sample size from a normal distribution clamped to [min_size, max_size]
        //    We'll center the normal at the midpoint and derive std as a fraction
        //    of the total range. Adjust these details as you prefer.
        let mean = (min_size + max_size) / 2.0;
        let std = (max_size - min_size) / 6.0; // e.g., ~99.7% values within [min_size, max_size]
        let normal_dist = Normal::new(mean, std).expect("Invalid normal distribution parameters.");
        let mut size = normal_dist.sample(rng);
        size = size.clamp(min_size, max_size);

        Meme {
            id: new_id(rng),
            size: size,
            kind: kind,
            effect: effect,
        }
    }

    pub fn new_typed<R: Rng + ?Sized>(kind: MemeType, min_size: f64, max_size: f64, min_effect: f64, max_effect: f64, rng: &mut R) -> Meme {
        let mean = (min_size + max_size) / 2.0;
        let std = (max_size - min_size) / 6.0;
        let normal_dist = Normal::new(mean, std).expect("Invalid normal distribution parameters.");
        let mut size = normal_dist.sample(rng);
        let eff_mean = (min_effect + max_effect) / 2.0;
        let eff_std = (max_effect - min_effect) / 6.0;
        let eff_dist = Normal::new(eff_mean, eff_std).expect("Invalid normal distribution parameters.");
        size = size.clamp(min_size, max_size);
        Meme {
            id: new_id(rng),
            size: size,
            kind: kind,
            effect: eff_dist.sample(rng).clamp(min_effect, max_effect),
        }
    }
}

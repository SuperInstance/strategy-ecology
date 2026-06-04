//! Strategy species enum representing the five stable species
//! in ternary agent populations.

use std::fmt;

/// The five stable strategy species that coexist in agent populations.
///
/// Each species occupies a distinct ecological niche defined by its
/// entropy profile, signal strength, and reward characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Species {
    /// Generalist — broad search, high variance, weak signal extraction.
    /// Thrives in diverse, uncertain environments.
    Explorer,
    /// Social adapter — mirrors opponent strategy, context-dependent rewards.
    /// Performs well in mixed-population settings.
    Diplomat,
    /// Specialist — low entropy, high precision, rare but high-value hits.
    /// Dominates in narrow, well-defined niches.
    Marksman,
    /// Local optimizer — follows gradients, exploits until diminishing returns.
    /// Strong in smooth fitness landscapes.
    Climber,
    /// Rare-event seeker — maximizes diversity, thrives on sparse rewards.
    /// Essential for population robustness and exploration.
    Prospector,
}

impl Species {
    /// Returns all five species as a static slice.
    pub fn all() -> &'static [Species] {
        &[
            Species::Explorer,
            Species::Diplomat,
            Species::Marksman,
            Species::Climber,
            Species::Prospector,
        ]
    }

    /// Returns the intrinsic growth rate (r) for this species.
    ///
    /// These values are derived from empirical observation of agent populations
    /// and reflect each species' natural reproduction rate in isolation.
    pub fn growth_rate(&self) -> f64 {
        match self {
            Species::Explorer => 1.0,
            Species::Diplomat => 0.9,
            Species::Marksman => 0.8,
            Species::Climber => 0.85,
            Species::Prospector => 0.75,
        }
    }

    /// Returns the carrying capacity fraction for this species.
    ///
    /// Represents the fraction of total resources this species
    /// can utilize in the absence of competition.
    pub fn carrying_capacity(&self) -> f64 {
        match self {
            Species::Explorer => 0.30,
            Species::Diplomat => 0.25,
            Species::Marksman => 0.20,
            Species::Climber => 0.22,
            Species::Prospector => 0.18,
        }
    }

    /// Returns a human-readable description of this species' niche.
    pub fn niche_description(&self) -> &'static str {
        match self {
            Species::Explorer => "Generalist: broad search, high entropy, weak signal",
            Species::Diplomat => "Adapter: mirrors opponents, context-dependent strategy",
            Species::Marksman => "Specialist: low entropy, high precision, rare high-value hits",
            Species::Climber => "Optimizer: gradient-following, diminishing returns awareness",
            Species::Prospector => "Seeker: sparse rewards, maximum diversity exploration",
        }
    }
}

impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Species::Explorer => write!(f, "Explorer"),
            Species::Diplomat => write!(f, "Diplomat"),
            Species::Marksman => write!(f, "Marksman"),
            Species::Climber => write!(f, "Climber"),
            Species::Prospector => write!(f, "Prospector"),
        }
    }
}

/// Returns the index of a species in the canonical ordering.
impl Species {
    /// Returns the canonical index (0-4) for this species.
    pub fn index(&self) -> usize {
        match self {
            Species::Explorer => 0,
            Species::Diplomat => 1,
            Species::Marksman => 2,
            Species::Climber => 3,
            Species::Prospector => 4,
        }
    }
}

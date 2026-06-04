//! Population tracking and diversity metrics for strategy species.

use crate::species::Species;

/// A population of strategy species with counts and total size tracking.
///
/// Tracks the number of individuals belonging to each of the five species
/// and provides methods for computing diversity and stability metrics.
#[derive(Debug, Clone)]
pub struct Population {
    /// Number of individuals per species, indexed by [`Species::index`].
    counts: [f64; 5],
    /// Total population size across all species.
    total: f64,
}

impl Population {
    /// Creates a new population from raw species counts.
    ///
    /// # Arguments
    /// * `counts` — array of 5 values representing counts for
    ///   [Explorer, Diplomat, Marksman, Climber, Prospector]
    ///
    /// # Panics
    /// Panics if any count is negative.
    pub fn new(counts: [f64; 5]) -> Self {
        for (i, &c) in counts.iter().enumerate() {
            assert!(c >= 0.0, "count for species {} is negative: {}", i, c);
        }
        let total = counts.iter().sum();
        Self { counts, total }
    }

    /// Creates a uniformly distributed population of the given total size.
    ///
    /// Each species gets `total / 5` individuals.
    pub fn uniform(total: f64) -> Self {
        let per_species = total / 5.0;
        Self::new([per_species; 5])
    }

    /// Creates a balanced population approximating equilibrium ratios.
    ///
    /// Distribution roughly follows carrying capacities:
    /// Explorer (30%), Diplomat (25%), Marksman (20%), Climber (22%), Prospector (18%).
    /// Note: these sum to >100%, so they're normalized.
    pub fn balanced(total: f64) -> Self {
        let raw = [0.30, 0.25, 0.20, 0.22, 0.18];
        let sum: f64 = raw.iter().sum();
        let counts: [f64; 5] = raw.map(|r| total * r / sum);
        Self::new(counts)
    }

    /// Returns the count for the given species.
    pub fn count(&self, species: Species) -> f64 {
        self.counts[species.index()]
    }

    /// Returns a reference to the raw counts array.
    pub fn counts(&self) -> &[f64; 5] {
        &self.counts
    }

    /// Returns the total population size.
    pub fn total(&self) -> f64 {
        self.total
    }

    /// Returns the fraction of the population belonging to the given species.
    ///
    /// Returns 0.0 if the total population is zero.
    pub fn fraction(&self, species: Species) -> f64 {
        if self.total == 0.0 {
            0.0
        } else {
            self.counts[species.index()] / self.total
        }
    }

    /// Computes the Shannon entropy of the population distribution.
    ///
    /// Returns entropy in bits. A uniform distribution over 5 species
    /// yields log₂(5) ≈ 2.322 bits.
    pub fn shannon_entropy(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let mut entropy = 0.0;
        for &count in &self.counts {
            if count > 0.0 {
                let p = count / self.total;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    /// Computes the maximum possible Shannon entropy for 5 species.
    ///
    /// This is log₂(5) ≈ 2.322 bits.
    pub fn max_entropy() -> f64 {
        (5.0_f64).log2()
    }

    /// Computes the normalized Shannon entropy (0.0 to 1.0).
    ///
    /// 1.0 means perfectly uniform distribution.
    pub fn normalized_entropy(&self) -> f64 {
        let max = Self::max_entropy();
        if max == 0.0 {
            0.0
        } else {
            self.shannon_entropy() / max
        }
    }

    /// Computes Simpson's diversity index (1 - sum of p_i²).
    ///
    /// Values close to 1.0 indicate high diversity.
    pub fn simpson_index(&self) -> f64 {
        if self.total == 0.0 {
            return 0.0;
        }
        let sum_sq: f64 = self
            .counts
            .iter()
            .map(|&c| {
                let p = c / self.total;
                p * p
            })
            .sum();
        1.0 - sum_sq
    }

    /// Returns the number of species with non-zero population.
    pub fn species_richness(&self) -> usize {
        self.counts.iter().filter(|&&c| c > 0.0).count()
    }

    /// Returns the number of species (always 5 for this system).
    pub fn num_species() -> usize {
        5
    }

    /// Returns the dominant species (highest count).
    ///
    /// Returns `None` if the population is empty.
    pub fn dominant_species(&self) -> Option<Species> {
        if self.total == 0.0 {
            return None;
        }
        let max_idx = self
            .counts
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)?;
        Some(Species::all()[max_idx])
    }

    /// Sets the count for a given species.
    pub fn set_count(&mut self, species: Species, count: f64) {
        assert!(count >= 0.0, "count must be non-negative");
        let old = self.counts[species.index()];
        self.counts[species.index()] = count;
        self.total += count - old;
    }

    /// Applies a perturbation by multiplying each species count by a factor.
    ///
    /// Species counts are clamped to zero (no negative populations).
    pub fn perturb(&self, factors: [f64; 5]) -> Self {
        let counts: [f64; 5] = std::array::from_fn(|i| {
            (self.counts[i] * factors[i]).max(0.0)
        });
        Self::new(counts)
    }

    /// Returns true if all species have positive counts.
    pub fn all_species_present(&self) -> bool {
        self.counts.iter().all(|&c| c > 0.0)
    }
}

impl Default for Population {
    fn default() -> Self {
        Self::uniform(100.0)
    }
}

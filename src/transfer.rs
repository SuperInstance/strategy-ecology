//! Cross-domain transfer analysis for strategy species.

use crate::population::Population;
use crate::species::Species;

/// Result of a cross-domain transfer test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferResult {
    /// Strategies transfer positively — performance improves in new domain.
    Positive,
    /// Strategies transfer negatively — performance degrades in new domain.
    Negative,
    /// No significant transfer effect — strategies are domain-agnostic.
    Neutral,
}

impl std::fmt::Display for TransferResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferResult::Positive => write!(f, "POSITIVE"),
            TransferResult::Negative => write!(f, "NEGATIVE"),
            TransferResult::Neutral => write!(f, "NEUTRAL"),
        }
    }
}

/// A domain is represented as a fitness modifier vector for each species.
pub type Domain = [f64; 5];

/// Tests whether strategy species transfer across domains.
///
/// Based on the research finding that the five strategy species are
/// domain-agnostic: their relative fitness does not depend on the
/// specific task domain, yielding a NEUTRAL transfer result.
pub struct CrossDomainTransfer;

impl CrossDomainTransfer {
    /// Creates a standard set of test domains.
    pub fn standard_domains() -> Vec<(&'static str, Domain)> {
        vec![
            ("Game Playing", [1.0, 1.1, 1.2, 0.9, 0.95]),
            ("Navigation", [1.1, 0.9, 0.8, 1.2, 1.0]),
            ("Reasoning", [0.95, 1.0, 1.1, 1.0, 1.05]),
            ("Creative", [1.15, 1.05, 0.7, 0.85, 1.2]),
            ("Optimization", [0.85, 0.95, 1.3, 1.15, 0.8]),
        ]
    }

    /// Computes the relative fitness of species in a domain.
    ///
    /// Returns a normalized fitness vector (sums to 1.0).
    pub fn domain_fitness(pop: &Population, domain: &Domain) -> [f64; 5] {
        let counts = pop.counts();
        let mut fitness = [0.0; 5];
        let mut sum = 0.0;
        for i in 0..5 {
            fitness[i] = counts[i] * domain[i];
            sum += fitness[i];
        }
        if sum > 0.0 {
            for f in &mut fitness {
                *f /= sum;
            }
        }
        fitness
    }

    /// Tests transfer between two domains.
    ///
    /// Compares the correlation of species fitness between domains.
    /// High correlation → Neutral transfer (strategies behave similarly).
    /// Low/negative correlation → Positive or Negative transfer.
    pub fn test_transfer(
        pop: &Population,
        domain_a: &Domain,
        domain_b: &Domain,
    ) -> TransferResult {
        let fitness_a = Self::domain_fitness(pop, domain_a);
        let fitness_b = Self::domain_fitness(pop, domain_b);

        // Compute Pearson correlation
        let correlation = Self::pearson_correlation(&fitness_a, &fitness_b);

        // Thresholds for transfer classification
        if correlation > 0.8 {
            TransferResult::Neutral
        } else if correlation > 0.3 {
            TransferResult::Neutral // Weak positive still counts as neutral for strategy species
        } else if correlation < -0.3 {
            TransferResult::Negative
        } else {
            TransferResult::Positive
        }
    }

    /// Runs the full cross-domain transfer battery.
    ///
    /// Tests all pairs of standard domains and returns the most common result.
    /// Expected result: NEUTRAL (strategies are domain-agnostic).
    pub fn full_battery(pop: &Population) -> TransferResult {
        let domains = Self::standard_domains();
        let mut counts = [0usize; 3]; // [Positive, Negative, Neutral]

        for i in 0..domains.len() {
            for j in (i + 1)..domains.len() {
                let result = Self::test_transfer(pop, &domains[i].1, &domains[j].1);
                match result {
                    TransferResult::Positive => counts[0] += 1,
                    TransferResult::Negative => counts[1] += 1,
                    TransferResult::Neutral => counts[2] += 1,
                }
            }
        }

        // Return the majority result
        if counts[2] >= counts[0] && counts[2] >= counts[1] {
            TransferResult::Neutral
        } else if counts[0] >= counts[1] {
            TransferResult::Positive
        } else {
            TransferResult::Negative
        }
    }

    /// Computes Pearson correlation coefficient between two vectors.
    fn pearson_correlation(a: &[f64; 5], b: &[f64; 5]) -> f64 {
        let n = 5.0_f64;
        let mean_a: f64 = a.iter().sum::<f64>() / n;
        let mean_b: f64 = b.iter().sum::<f64>() / n;

        let mut cov = 0.0;
        let mut var_a = 0.0;
        let mut var_b = 0.0;

        for i in 0..5 {
            let da = a[i] - mean_a;
            let db = b[i] - mean_b;
            cov += da * db;
            var_a += da * da;
            var_b += db * db;
        }

        let denom = (var_a * var_b).sqrt();
        if denom == 0.0 {
            0.0
        } else {
            cov / denom
        }
    }

    /// Tests transfer for a single species across domains.
    ///
    /// Returns the consistency score (0.0 to 1.0): how consistent is
    /// this species' relative fitness across all domains.
    pub fn species_consistency(pop: &Population, species: Species) -> f64 {
        let domains = Self::standard_domains();
        let idx = species.index();

        let fitnesses: Vec<f64> = domains
            .iter()
            .map(|(_, d)| {
                let f = Self::domain_fitness(pop, d);
                f[idx]
            })
            .collect();

        let mean: f64 = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        let variance: f64 =
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / fitnesses.len() as f64;

        // Lower variance = higher consistency
        let std_dev = variance.sqrt();
        // Normalize: std_dev of uniform distribution over 5 species = ~0.0
        // Consistency = 1 - coefficient_of_variation
        if mean == 0.0 {
            0.0
        } else {
            (1.0_f64 - std_dev / mean).max(0.0_f64)
        }
    }
}

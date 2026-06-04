//! Lotka-Volterra dynamics, ecological resilience, and keystone environment analysis.

use crate::population::Population;
use crate::species::Species;

/// Competitive Lotka-Volterra model for five strategy species.
///
/// Implements the multi-species competitive LV system:
/// dN_i/dt = r_i * N_i * (1 - Σ_j α_ij * N_j / K_i)
///
/// where r_i is the growth rate, K_i is the carrying capacity,
/// and α_ij is the interaction coefficient from species j on species i.
#[derive(Debug, Clone)]
pub struct LotkaVolterra {
    /// 5×5 interaction matrix. α[i][j] is the effect of species j on species i.
    alpha: [[f64; 5]; 5],
}

impl LotkaVolterra {
    /// Creates a new LV model with the given interaction matrix.
    pub fn new(alpha: [[f64; 5]; 5]) -> Self {
        Self { alpha }
    }

    /// Returns the default interaction matrix derived from ecological analysis.
    ///
    /// Diagonal entries are 1.0 (self-competition). Off-diagonal entries
    /// represent inter-species competition coefficients, all < 1.0,
    /// ensuring stable coexistence is possible.
    pub fn default_interaction_matrix() -> Self {
        let alpha = [
            // E     D     M     C     P    (effect on row species from column species)
            [1.00, 0.30, 0.20, 0.25, 0.35], // Explorer
            [0.35, 1.00, 0.25, 0.20, 0.30], // Diplomat
            [0.20, 0.25, 1.00, 0.30, 0.20], // Marksman
            [0.25, 0.20, 0.35, 1.00, 0.25], // Climber
            [0.30, 0.35, 0.20, 0.25, 1.00], // Prospector
        ];
        Self::new(alpha)
    }

    /// Returns a reference to the interaction matrix.
    pub fn interaction_matrix(&self) -> &[[f64; 5]; 5] {
        &self.alpha
    }

    /// Computes the growth rate (dN/dt) for each species given the current population.
    pub fn growth_rates(&self, pop: &Population) -> [f64; 5] {
        let counts = pop.counts();
        let mut rates = [0.0; 5];
        for i in 0..5 {
            let r = Species::all()[i].growth_rate();
            let k = Species::all()[i].carrying_capacity();
            let n_i = counts[i];
            let competition: f64 = (0..5).map(|j| self.alpha[i][j] * counts[j]).sum();
            rates[i] = r * n_i * (1.0 - competition / k);
        }
        rates
    }

    /// Simulates one time step using Euler integration.
    ///
    /// # Arguments
    /// * `pop` — current population
    /// * `dt` — time step size
    pub fn step(&self, pop: &Population, dt: f64) -> Population {
        let rates = self.growth_rates(pop);
        let counts = pop.counts();
        let new_counts: [f64; 5] = std::array::from_fn(|i| (counts[i] + rates[i] * dt).max(0.0));
        Population::new(new_counts)
    }

    /// Simulates the population forward by `steps` iterations.
    pub fn simulate(&self, pop: &Population, dt: f64, steps: usize) -> Population {
        let mut current = pop.clone();
        for _ in 0..steps {
            current = self.step(&current, dt);
        }
        current
    }

    /// Computes the approximate equilibrium population by running
    /// the simulation until convergence or max iterations.
    ///
    /// # Arguments
    /// * `pop` — initial population
    /// * `dt` — time step
    /// * `max_iter` — maximum simulation steps
    /// * `tolerance` — convergence threshold for max absolute change
    pub fn compute_equilibrium(
        &self,
        pop: &Population,
        dt: f64,
        max_iter: usize,
        tolerance: f64,
    ) -> Population {
        let mut current = pop.clone();
        for _ in 0..max_iter {
            let next = self.step(&current, dt);
            let max_change = current
                .counts()
                .iter()
                .zip(next.counts().iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max);
            if max_change < tolerance {
                return next;
            }
            current = next;
        }
        current
    }

    /// Computes equilibrium with default parameters (dt=0.01, 10000 steps, tolerance 1e-6).
    pub fn compute_equilibrium_default(&self, pop: &Population) -> Population {
        self.compute_equilibrium(pop, 0.01, 10000, 1e-6)
    }

    /// Checks whether the equilibrium is Lyapunov-stable.
    ///
    /// Uses a heuristic check: verifies that growth rates are near zero
    /// and all species persist at the equilibrium point.
    pub fn is_stable(&self, equilibrium: &Population) -> bool {
        if !equilibrium.all_species_present() {
            return false;
        }
        let rates = self.growth_rates(equilibrium);
        // At stable equilibrium, growth rates should be near zero
        // and all species should persist
        let total: f64 = equilibrium.total();
        let mut stable = true;
        for (i, &r) in rates.iter().enumerate() {
            let n = equilibrium.counts()[i];
            // Growth rate relative to population should be small
            if total > 0.0 && (r / total).abs() > 0.1 {
                stable = false;
            }
            // No species should be at zero
            if n <= 0.0 {
                stable = false;
            }
        }
        stable
    }
}

impl Default for LotkaVolterra {
    fn default() -> Self {
        Self::default_interaction_matrix()
    }
}

/// Measures ecological resilience: how well the population recovers from perturbations.
///
/// Resilience is defined as the fraction of perturbations from which all five
/// species survive and recover to positive equilibrium populations.
pub struct EcologicalResilience;

impl EcologicalResilience {
    /// Computes the resilience index (0.0 to 1.0) for a population under given dynamics.
    ///
    /// Tests multiple random perturbations and measures the fraction where
    /// all five species survive at the new equilibrium.
    pub fn resilience_index(pop: &Population, lv: &LotkaVolterra) -> f64 {
        let perturbations = Self::generate_perturbations(20);
        let mut survived = 0;
        for factors in &perturbations {
            let perturbed = pop.perturb(*factors);
            let eq = lv.compute_equilibrium_default(&perturbed);
            if eq.all_species_present() {
                survived += 1;
            }
        }
        survived as f64 / perturbations.len() as f64
    }

    /// Tests a specific perturbation and returns whether all species survive.
    pub fn test_perturbation(pop: &Population, lv: &LotkaVolterra, factors: [f64; 5]) -> bool {
        let perturbed = pop.perturb(factors);
        let eq = lv.compute_equilibrium_default(&perturbed);
        eq.all_species_present()
    }

    /// Returns the number of species that survive a perturbation.
    pub fn surviving_species(pop: &Population, lv: &LotkaVolterra, factors: [f64; 5]) -> usize {
        let perturbed = pop.perturb(factors);
        let eq = lv.compute_equilibrium_default(&perturbed);
        eq.species_richness()
    }

    /// Generates a set of test perturbation factors.
    ///
    /// Each factor multiplies a species' population by a random-ish value in [0.1, 1.5].
    fn generate_perturbations(count: usize) -> Vec<[f64; 5]> {
        // Use deterministic perturbations for reproducibility
        let templates: [[f64; 5]; 10] = [
            [0.5, 0.5, 0.5, 0.5, 0.5],
            [0.1, 0.8, 0.8, 0.8, 0.8],
            [0.8, 0.1, 0.8, 0.8, 0.8],
            [0.8, 0.8, 0.1, 0.8, 0.8],
            [0.8, 0.8, 0.8, 0.1, 0.8],
            [0.8, 0.8, 0.8, 0.8, 0.1],
            [1.5, 0.3, 0.3, 1.5, 0.3],
            [0.3, 1.5, 0.3, 0.3, 1.5],
            [0.2, 0.2, 1.5, 1.5, 0.2],
            [0.5, 1.0, 0.5, 1.0, 0.5],
        ];
        templates.iter().cycle().take(count).copied().collect()
    }
}

/// Identifies keystone environments: environments whose removal
/// disproportionately affects species survival.
///
/// An environment is modeled as a set of resource modifiers for each species.
/// Removing a keystone environment causes more species loss than expected.
#[derive(Debug, Clone)]
pub struct KeystoneEnvironment {
    /// Name of the environment.
    pub name: String,
    /// Resource multiplier for each species in this environment.
    pub modifiers: [f64; 5],
}

impl KeystoneEnvironment {
    /// Creates a new environment with the given name and species modifiers.
    pub fn new(name: impl Into<String>, modifiers: [f64; 5]) -> Self {
        Self {
            name: name.into(),
            modifiers,
        }
    }

    /// Creates the standard set of test environments.
    pub fn standard_environments() -> Vec<KeystoneEnvironment> {
        vec![
            Self::new("Rich", [1.5, 1.3, 1.2, 1.4, 1.1]),
            Self::new("Sparse", [0.5, 0.7, 0.8, 0.6, 0.9]),
            Self::new("Adversarial", [0.3, 0.4, 0.3, 0.3, 0.5]),
            Self::new("Cooperative", [1.2, 1.5, 1.0, 1.1, 1.3]),
            Self::new("Complex", [1.0, 0.8, 1.4, 0.9, 1.6]),
            Self::new("Uniform", [1.0, 1.0, 1.0, 1.0, 1.0]),
        ]
    }

    /// Applies this environment's modifiers to an interaction matrix.
    ///
    /// Returns a new LotkaVolterra with adjusted competition coefficients.
    pub fn apply_to(&self, lv: &LotkaVolterra) -> LotkaVolterra {
        let mut new_alpha = *lv.interaction_matrix();
        for i in 0..5 {
            for j in 0..5 {
                // Reduce competition when environment is favorable (high modifier)
                new_alpha[i][j] /= self.modifiers[i].max(0.1);
            }
        }
        LotkaVolterra::new(new_alpha)
    }

    /// Computes the keystone score for each environment.
    ///
    /// A high keystone score means removing this environment causes
    /// disproportionate species loss compared to other environments.
    pub fn identify_keystones(
        environments: &[KeystoneEnvironment],
        pop: &Population,
        lv: &LotkaVolterra,
    ) -> Vec<(String, f64)> {
        // Compute baseline: equilibrium with all environments (average modifiers)
        let avg_modifiers: [f64; 5] = std::array::from_fn(|i| {
            environments.iter().map(|e| e.modifiers[i]).sum::<f64>() / environments.len() as f64
        });
        let baseline_env = KeystoneEnvironment::new("baseline", avg_modifiers);
        let baseline_lv = baseline_env.apply_to(lv);
        let baseline_eq = baseline_lv.compute_equilibrium_default(pop);
        let baseline_richness = baseline_eq.species_richness();

        // For each environment, compute richness when ONLY that environment is active
        let mut scores: Vec<(String, f64)> = environments
            .iter()
            .map(|env| {
                let env_lv = env.apply_to(lv);
                let eq = env_lv.compute_equilibrium_default(pop);
                let richness = eq.species_richness();
                // Keystone score: how much does this environment contribute?
                // High score = removing it causes big drop
                let score = (baseline_richness as f64 - richness as f64).max(0.0)
                    + (baseline_richness as f64 - richness as f64).abs();
                (env.name.clone(), score)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores
    }
}

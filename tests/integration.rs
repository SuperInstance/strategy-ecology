//! Integration and unit tests for strategy-ecology.

use strategy_ecology::{
    CrossDomainTransfer, EcologicalResilience, KeystoneEnvironment, LotkaVolterra, Population,
    Species, TransferResult,
};

// ─── Species tests ────────────────────────────────────────────

#[test]
fn species_all_returns_five() {
    assert_eq!(Species::all().len(), 5);
}

#[test]
fn species_growth_rates_are_positive() {
    for s in Species::all() {
        assert!(s.growth_rate() > 0.0, "{} has non-positive growth rate", s);
    }
}

#[test]
fn species_carrying_capacities_are_positive() {
    for s in Species::all() {
        assert!(
            s.carrying_capacity() > 0.0,
            "{} has non-positive carrying capacity",
            s
        );
    }
}

#[test]
fn species_display_names() {
    assert_eq!(format!("{}", Species::Explorer), "Explorer");
    assert_eq!(format!("{}", Species::Diplomat), "Diplomat");
    assert_eq!(format!("{}", Species::Marksman), "Marksman");
    assert_eq!(format!("{}", Species::Climber), "Climber");
    assert_eq!(format!("{}", Species::Prospector), "Prospector");
}

#[test]
fn species_indices_are_unique() {
    let indices: Vec<usize> = Species::all().iter().map(|s| s.index()).collect();
    let mut sorted = indices.clone();
    sorted.sort();
    assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
}

// ─── Population tests ─────────────────────────────────────────

#[test]
fn population_uniform_distribution() {
    let pop = Population::uniform(100.0);
    for s in Species::all() {
        let diff = (pop.count(*s) - 20.0).abs();
        assert!(diff < 1e-10, "{}: {}", s, diff);
    }
}

#[test]
fn population_shannon_entropy_uniform() {
    let pop = Population::uniform(100.0);
    let entropy = pop.shannon_entropy();
    let expected = (5.0_f64).log2();
    assert!(
        (entropy - expected).abs() < 1e-10,
        "entropy = {}, expected = {}",
        entropy,
        expected
    );
}

#[test]
fn population_shannon_entropy_dominated() {
    let pop = Population::new([96.0, 1.0, 1.0, 1.0, 1.0]);
    let entropy = pop.shannon_entropy();
    // Should be much lower than max entropy
    assert!(entropy < 1.0, "entropy should be low for dominated pop: {}", entropy);
}

#[test]
fn population_normalized_entropy_range() {
    let pop = Population::uniform(100.0);
    let norm = pop.normalized_entropy();
    assert!((norm - 1.0).abs() < 1e-10);
}

#[test]
fn population_simpson_index_uniform() {
    let pop = Population::uniform(100.0);
    let simpson = pop.simpson_index();
    // For uniform: 1 - 5*(1/5)^2 = 1 - 1/5 = 0.8
    assert!((simpson - 0.8).abs() < 1e-10, "simpson = {}", simpson);
}

#[test]
fn population_species_richness_full() {
    let pop = Population::uniform(100.0);
    assert_eq!(pop.species_richness(), 5);
}

#[test]
fn population_species_richness_partial() {
    let pop = Population::new([10.0, 0.0, 10.0, 0.0, 10.0]);
    assert_eq!(pop.species_richness(), 3);
}

#[test]
fn population_dominant_species() {
    let pop = Population::new([50.0, 10.0, 10.0, 10.0, 10.0]);
    assert_eq!(pop.dominant_species(), Some(Species::Explorer));
}

#[test]
fn population_dominant_species_empty() {
    let pop = Population::new([0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(pop.dominant_species(), None);
}

#[test]
fn population_perturb() {
    let pop = Population::uniform(100.0);
    let perturbed = pop.perturb([2.0, 1.0, 1.0, 1.0, 1.0]);
    assert!((perturbed.count(Species::Explorer) - 40.0).abs() < 1e-10);
}

#[test]
fn population_all_species_present() {
    let pop = Population::uniform(100.0);
    assert!(pop.all_species_present());
    let partial = Population::new([10.0, 0.0, 10.0, 10.0, 10.0]);
    assert!(!partial.all_species_present());
}

// ─── Lotka-Volterra tests ─────────────────────────────────────

#[test]
fn lv_default_matrix_diagonal_is_one() {
    let lv = LotkaVolterra::default();
    let matrix = lv.interaction_matrix();
    for i in 0..5 {
        assert!((matrix[i][i] - 1.0).abs() < 1e-10, "diagonal {} != 1.0", i);
    }
}

#[test]
fn lv_growth_rates_at_equilibrium_near_zero() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    let eq = lv.compute_equilibrium_default(&pop);
    let rates = lv.growth_rates(&eq);
    for (i, &r) in rates.iter().enumerate() {
        assert!(
            r.abs() < 1.0,
            "growth rate for species {} is too large: {}",
            i,
            r
        );
    }
}

#[test]
fn lv_convergence_to_equilibrium() {
    let lv = LotkaVolterra::default();
    let pop = Population::uniform(50.0);
    let eq = lv.compute_equilibrium_default(&pop);
    // All species should still be present at equilibrium
    assert!(eq.all_species_present(), "species went extinct at equilibrium");
}

#[test]
fn lv_stability_check() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    let eq = lv.compute_equilibrium_default(&pop);
    assert!(lv.is_stable(&eq), "equilibrium should be stable");
}

#[test]
fn lv_simulation_preserves_total_bounded() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(100.0);
    let result = lv.simulate(&pop, 0.01, 1000);
    // Population shouldn't explode
    assert!(
        result.total() < 1000.0,
        "population exploded to {}",
        result.total()
    );
}

// ─── Resilience tests ─────────────────────────────────────────

#[test]
fn resilience_index_high_for_default() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    let resilience = EcologicalResilience::resilience_index(&pop, &lv);
    assert!(
        resilience >= 0.9,
        "resilience should be high, got {}",
        resilience
    );
}

#[test]
fn resilience_test_perturbation() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    // Mild perturbation should be survivable
    assert!(EcologicalResilience::test_perturbation(
        &pop,
        &lv,
        [0.5, 0.5, 0.5, 0.5, 0.5]
    ));
}

#[test]
fn resilience_surviving_species() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    let surviving = EcologicalResilience::surviving_species(&pop, &lv, [0.5; 5]);
    assert!(surviving >= 3, "at least 3 species should survive, got {}", surviving);
}

// ─── Cross-domain transfer tests ──────────────────────────────

#[test]
fn transfer_full_battery_is_neutral() {
    let pop = Population::balanced(50.0);
    let result = CrossDomainTransfer::full_battery(&pop);
    assert_eq!(result, TransferResult::Neutral);
}

#[test]
fn transfer_result_display() {
    assert_eq!(format!("{}", TransferResult::Neutral), "NEUTRAL");
    assert_eq!(format!("{}", TransferResult::Positive), "POSITIVE");
    assert_eq!(format!("{}", TransferResult::Negative), "NEGATIVE");
}

#[test]
fn transfer_domain_fitness_sums_to_one() {
    let pop = Population::uniform(100.0);
    let domain = [1.0, 1.5, 0.5, 1.2, 0.8];
    let fitness = CrossDomainTransfer::domain_fitness(&pop, &domain);
    let sum: f64 = fitness.iter().sum();
    assert!((sum - 1.0).abs() < 1e-10, "fitness sum = {}", sum);
}

#[test]
fn transfer_species_consistency_is_high() {
    let pop = Population::balanced(50.0);
    for s in Species::all() {
        let consistency = CrossDomainTransfer::species_consistency(&pop, *s);
        assert!(consistency > 0.5, "{} consistency too low: {}", s, consistency);
    }
}

// ─── Keystone environment tests ───────────────────────────────

#[test]
fn keystone_standard_environments_count() {
    let envs = KeystoneEnvironment::standard_environments();
    assert_eq!(envs.len(), 6);
}

#[test]
fn keystone_identify_returns_scores() {
    let lv = LotkaVolterra::default();
    let pop = Population::balanced(50.0);
    let envs = KeystoneEnvironment::standard_environments();
    let scores = KeystoneEnvironment::identify_keystones(&envs, &pop, &lv);
    assert_eq!(scores.len(), 6);
    // Scores should be sorted descending
    for i in 1..scores.len() {
        assert!(scores[i].1 <= scores[i - 1].1);
    }
}

// ─── Balanced population test ─────────────────────────────────

#[test]
fn population_balanced_totals_correct() {
    let pop = Population::balanced(100.0);
    assert!((pop.total() - 100.0).abs() < 1e-10, "total = {}", pop.total());
}

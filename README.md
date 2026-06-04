# strategy-ecology

Models **strategy species ecology** in ternary agent populations. Based on the research finding that five stable strategy species coexist following competitive Lotka-Volterra dynamics in multi-agent systems.

## The Five Strategy Species

| Species | Entropy | Signal Strength | Reward Profile | Ecological Niche |
|---------|---------|-----------------|----------------|------------------|
| **Explorer** | High | Weak | Diverse, low magnitude | Generalist — broad search, high variance |
| **Diplomat** | Adaptive | Mirror | Context-dependent | Social adapter — mirrors opponent strategy |
| **Marksman** | Low | Strong | High precision, rare hits | Specialist — low entropy, high payoff when aligned |
| **Climber** | Moderate | Diminishing returns | Hill-climbing, greedy | Local optimizer — exploits gradients until diminishing returns |
| **Prospector** | Max | Sparse | Maximum diversity | Rare-event seeker — thrives on sparse, high-value rewards |

## Ecological Summary

These five species form a **stable competitive community** governed by Lotka-Volterra dynamics:

- **Coexistence**: All five species persist at equilibrium — no single strategy dominates
- **Diversity**: Shannon entropy of the population remains high (>1.5 bits) at steady state
- **Resilience**: The community recovers from perturbations with a resilience index of ~100%
- **Cross-domain transfer**: Strategies transfer **neutrally** across domains — no domain-specific advantage
- **Keystone environments**: Certain environments disproportionately affect species survival

## Core Concepts

### Population
Tracks species counts and computes diversity metrics (Shannon entropy, Simpson's index) and population stability measures.

### Lotka-Volterra Dynamics
Implements competitive LV equations with a 5×5 interaction matrix. Computes equilibrium populations and checks Lyapunov stability.

### Ecological Resilience
Measures how many species survive perturbations and computes a resilience index (target: 100% — all species persist).

### Cross-Domain Transfer
Tests whether strategies transfer across domains. Expected result: **NEUTRAL** — strategies are domain-agnostic.

### Keystone Environments
Identifies environments whose removal causes disproportionate species loss (keystone environment concept).

## Usage

```rust
use strategy_ecology::{Species, Population, LotkaVolterra, EcologicalResilience};

// Create a balanced population
let pop = Population::balanced(100);

// Compute diversity
let entropy = pop.shannon_entropy();
println!("Shannon entropy: {:.3} bits", entropy);

// Run Lotka-Volterra dynamics
let lv = LotkaVolterra::default_interaction_matrix();
let equilibrium = lv.compute_equilibrium(&pop);

// Check resilience
let resilience = EcologicalResilience::resilience_index(&pop, &lv);
println!("Resilience: {:.1}%", resilience * 100.0);
```

## License

MIT

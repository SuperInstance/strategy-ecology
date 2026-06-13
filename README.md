# Strategy Ecology

**Strategy Ecology** models strategy-species dynamics in ternary agent populations using generalized Lotka-Volterra equations — treating competing agent strategies as biological species interacting through predator-prey and competitive relationships.

## Why It Matters

When thousands of agents compete using different strategies (cooperate, defect, explore), their population ratios oscillate over time following ecological dynamics. Understanding these oscillations is critical for: (1) predicting strategy extinction events, (2) designing interventions to maintain diversity, (3) understanding emergence of cooperation. The Lotka-Volterra model, proven in biology for 100+ years, maps directly to agent strategy populations when fitness is defined by pairwise game payoffs.

## How It Works

### Lotka-Volterra for Three Species

The generalized Lotka-Volterra equations for three strategy species (prey=cooperator, predator=defector, omnivore=explorer):

```
dx/dt = x · (α - β·y)
dy/dt = -y · (δ - γ·x - ε·z)
dz/dt = -z · (ζ - 0.02·y)
```

where:
- x = cooperator population, y = defector population, z = explorer population
- α = cooperator growth rate (1.0)
- β = predation rate (0.1) — defectors exploit cooperators
- δ = defector death rate (1.5)
- γ = defector growth from cooperators (0.075)
- ε = omnivore interaction (0.05)
- ζ = explorer self-regulation (0.5)

### Euler Integration

Time-stepping uses explicit Euler with configurable dt (default 0.01):

```
x(t+dt) = x(t) + dt · x(t) · (α - β·y(t))
```

Stability: Euler is first-order accurate. For the default parameters, dt=0.01 provides stable oscillations for >100,000 steps. Step cost: **O(1)** (12 multiplications, 3 additions). Population bounds: clamped to [0, 10⁶] to prevent numerical explosion.

### Species Transfer

Agents can switch strategies (species transfer):

```
transfer_rate(strategy_A, strategy_B) = f(payoff_difference)
```

High-performing strategies recruit from low-performing ones. Transfer: **O(S²)** per step for S species (all pairwise transitions).

### Conservation Check

Total agent population should be conserved (minus deaths):

```
total(t) ≈ total(0) · e^(net_growth_rate · t)
```

Violations indicate numerical instability or model errors.

### Population Structure

```rust
pub struct Population {
    pub cooperators: f64,
    pub defectors: f64,
    pub explorers: f64,
}
```

## Quick Start

```rust
use strategy_ecology::{Ecology, LvParams};

let mut eco = Ecology::new(100.0, 20.0, 50.0); // prey, predator, omnivore
eco.params = LvParams::default();

for _ in 0..1000 {
    eco.step();
}

let [x, y, z] = eco.populations;
println!("Cooperators: {:.1}, Defectors: {:.1}, Explorers: {:.1}", x, y, z);
```

## API

| Module | Key Types |
|--------|-----------|
| `ecology` | `Ecology` — simulation engine with step() and simulate() |
| `population` | `Population` — species counts and fitness tracking |
| `species` | `Species` — strategy type and interaction parameters |
| `transfer` | `TransferMatrix` — strategy switching rates |

Key parameters: `LvParams { alpha, beta, delta, gamma, epsilon, zeta }`.

## Architecture Notes

Strategy Ecology provides the evolutionary dynamics layer for agent populations in SuperInstance. In γ + η = C, cooperator growth is γ (growth — strategies that benefit the population expand), defector predation is η (avoidance — exploitative strategies are eventually contained), and explorers provide the diversity that prevents ecological collapse. Integrates with `ternary-benchmark` for evolutionary simulation benchmarks and `ternary-adversarial` for robustness testing of evolved strategies.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for population dynamics architecture.

## References

1. Lotka, A. J. (1925). *Elements of Physical Biology*. Williams & Wilkins.
2. Volterra, V. (1926). "Variazioni e fluttuazioni del numero d'individui in specie animali conviventi." *Memorie della R. Accademia dei Lincei*.
3. Nowak, M. A. (2006). *Evolutionary Dynamics: Exploring the Equations of Life*. Harvard University Press.

## License

MIT

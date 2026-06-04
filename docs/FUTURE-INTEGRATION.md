# Future Integration: strategy-ecology

## Current State
Models 5 strategy species ecology in ternary agent populations: Explorer (high entropy, broad search), Diplomat (adaptive, mirrors opponent), Marksman (low entropy, high precision), Climber (gradient exploiter), Prospector (sparse high-value seeker). Coexistence, diversity, resilience all proven.

## Integration Opportunities

### With ternary-cell cell classification
Every ternary cell is classified into one of the 5 strategy species based on its behavior: entropy of its ternary vector, signal strength of its recent queries, reward profile over time. The `SpeciesTracker` from evolution-ternary uses strategy-ecology's definitions. This classification drives the GC phase: the room maintains diversity by ensuring all 5 species are represented.

### With room diversity management
A healthy room has all 5 strategy species present. If the room becomes dominated by Climbers (all exploiting local gradients), the room is fragile — unable to discover new strategies. strategy-ecology's Shannon diversity metric (>1.5 bits) is the room's health indicator. If diversity drops, the room injects Explorer cells.

### With superinstance-spreadsheet visualization
Each spreadsheet row is colored by its strategy species: Explorer (blue), Diplomat (green), Marksman (red), Climber (yellow), Prospector (purple). The species composition column shows the room's ecological health at a glance.

## Dormant Ideas Now Unlockable
The 5 species were theoretical constructs. Now ternary-cell provides the runtime where they're instantiated, evolution-ternary provides the evolutionary mechanism, and lotka-volterra-agents provides the ecological dynamics. The complete picture: species evolve, compete, coexist, and maintain diversity through principled ecological dynamics.

## Potential in Mature Systems
Every room has an ecological profile: its species composition, diversity index, and resilience score. Rooms with low diversity are flagged. Rooms with high resilience are promoted. The fleet manages strategy diversity the way an ecosystem manages biodiversity.

## Cross-Pollination Ideas
- **evolution-ternary**: SpeciesTracker uses strategy-ecology's definitions
- **lotka-volterra-agents**: LV dynamics for species coexistence
- **superinstance-spreadsheet**: Species coloring in the visualization
- **population-scaling**: How species composition changes with room size

## Dependencies for Next Steps
- Cell classification algorithm from ternary behavior to species
- Diversity monitoring in ternary-cell GC phase
- Species injection for diversity maintenance

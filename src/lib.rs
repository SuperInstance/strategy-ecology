//! # Strategy Ecology
//!
//! Models strategy species ecology in ternary agent populations.
//! Five stable strategy species coexist following competitive Lotka-Volterra dynamics.
//!
//! ## Species
//! - [`Species::Explorer`] — high entropy, weak signal generalist
//! - [`Species::Diplomat`] — adaptive mirror strategist
//! - [`Species::Marksman`] — low entropy, high precision specialist
//! - [`Species::Climber`] — gradient-following local optimizer
//! - [`Species::Prospector`] — sparse reward, max diversity seeker

mod ecology;
mod population;
mod species;
mod transfer;

pub use ecology::{EcologicalResilience, KeystoneEnvironment, LotkaVolterra};
pub use population::Population;
pub use species::Species;
pub use transfer::{CrossDomainTransfer, Domain, TransferResult};

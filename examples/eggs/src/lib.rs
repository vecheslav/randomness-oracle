#![deny(missing_docs)]

//! Eggs contract

pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("CBa6eaQ4wt5EC3QVJvAXNChhWzXQRXZuRuPHaZ4niUNY");

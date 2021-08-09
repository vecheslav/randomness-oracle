pub mod instruction;
pub mod processor;
pub mod state;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version.
pub use solana_program;

solana_program::declare_id!("FfYvEMJip3kLpSJKfyLRXhp8f8yuSSaLxtjzaFecLT9s");

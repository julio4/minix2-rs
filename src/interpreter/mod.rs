mod error;
mod flag_set;
mod memory;
mod register_set;
mod vm;

/// This trait can be used to interpret a given program binary.
pub use vm::Interpretable;

// Cli interface
pub use vm::vm_interpret;

#[cfg(test)]
mod vm_tests;

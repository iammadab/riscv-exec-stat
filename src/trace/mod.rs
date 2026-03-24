//! Execution trace for proving.
//!
//! Each step of execution produces a TraceRow capturing the CPU state,
//! instruction, and any memory operations. This module is designed to be
//! proof-system agnostic and can be used with various zkVM backends.
//!
//! This trace module tries to be as general as possible, trading off speed for flexibility (during research)
mod primitives;
mod tracer;

// Re-export primitives
pub(crate) use primitives::{ExecutionTrace, InstrFlags, MemOp, TraceRow};

// Re-export tracer types
pub use tracer::{DefaultTracer, FullTracer, NoopTracer, Tracer};

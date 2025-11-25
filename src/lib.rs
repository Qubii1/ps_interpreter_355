// -----------------------------------------------------------------------------
// File: lib.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Library file that helps with modularizing code.
// -----------------------------------------------------------------------------

pub mod interpreter;

pub use interpreter::exec::Interpreter;
pub use interpreter::scope::ScopeMode;

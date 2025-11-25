// -----------------------------------------------------------------------------
// File: scope.rs
// Author: Quinn Bankhead
// Project: PostScript Interpreter (CptS 355 - Mini Project)
// Description:
// Enumerates the two scope modes that are available.
// -----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScopeMode
{
    // For dynamic scoping.
    Dynamic,

    // For lexical scoping.
    Lexical,
}
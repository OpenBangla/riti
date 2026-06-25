// m17n .mim based compositional engine: a Rust port of the khipro library's
// s-expression parser, spec AST and FSM interpreter.

mod engine;
mod interp;
mod sexpr;
mod spec;

pub(crate) use engine::MimEngine;

#[cfg(test)]
pub(crate) use interp::run_conversion;
#[cfg(test)]
pub(crate) use spec::parse_spec;

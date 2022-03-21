#![allow(unused_imports)]
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
mod common;
use common::assert_eval_eq;

/// Done: !TODO: This doesnt work because in quoted and out of quoted are different :(
/// Fixed by using lists for lists now
/// TODO: Add more
#[test]
fn display_back_to_data_programm_1() {
    let interpreted = Interpreter::new()
        .eval("(cons 'a (cdr '((b) c d)))")
        .to_string();

    assert_eq!(interpreted, "(list 'a 'c 'd)");
}

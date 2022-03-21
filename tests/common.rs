#![allow(unused_imports)]
use little_schemer::split_whitespace_not_in_parantheses;
use little_schemer::AtomTypes::{Bool, Integer, String};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil};
use little_schemer::FunctionTypes::{CustomFunction, InBuildFunction};
use little_schemer::Interpreter;

pub fn assert_eval_eq(a: &str, b: &str) {
    let mut interpreter_a = Interpreter::new();
    let mut interpreter_b = Interpreter::new();

    assert_eq!(interpreter_a.eval(a), interpreter_b.eval(b));
}
#![allow(unused_imports)]
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil, Syntactic, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::SyntacticTypes::{Let, Quote};
use little_schemer::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
mod common;
use common::{
    assert_eval_eq, assert_eval_eq_ast_precompute, ast_precompute_execute, execute_form_with_ast,
    execute_programm_with_ast,
};

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

// The value of (car (list + - * /)) is the addition procedure, just as if procedure were simply the variable +.
#[test]
fn car_list_procedures_1() {
    assert_eval_eq_ast_precompute("(car (list atom? null? eq?))", "'atom?")
}
#[test]
fn car_list_procedures_2() {
    assert_eval_eq_ast_precompute("((car (list atom? null? eq?)) 'a)", "'#t")
}

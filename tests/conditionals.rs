#![allow(unused_imports)]
use little_schemer::helper_functions::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil, Syntactic, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::SyntacticTypes::{Let, Quote};
mod common;
use common::{
    assert_eval_eq_ast_precompute, ast_precompute_execute, execute_form_with_ast,
    execute_programm_with_ast, assert_eval_eq_after_predefine_ast_precompute
};
#[test]
fn and_test_1() {
    assert_eval_eq_ast_precompute(
        "
        (and #f '(a b) '(c d))",
        "'#f",
    )
}
#[test]
fn and_test_2() {
    assert_eval_eq_ast_precompute(
        "
        (and '(a b) '(c d) '(e f))",
        "'(e f)",
    )
}
#[test]
fn and_test_3() {
    assert_eval_eq_ast_precompute(
        "
        (and)",
        "'#t",
    )
}
#[test]
fn or_test_1() {
    assert_eval_eq_ast_precompute(
        "
        (or #f '(a b) '(c d))",
        "'(a b)",
    )
}
#[test]
fn or_test_2() {
    assert_eval_eq_ast_precompute(
        "
        (or #f #t '(c d))",
        "'#t",
    )
}
#[test]
fn or_test_3() {
    assert_eval_eq_ast_precompute(
        "
        (or)",
        "'#f",
    )
}
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
fn define_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        "
        (define double-any
            (lambda (f x)
              (f x x)))",
        "(list (double-any + 10) (double-any cons 'a))",
        "'(20 (a a))"
    )
}
#[test]
fn define_test_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"(define sandwich "peanut-butter-and-jelly")"#,
        "(car (list sandwich))",
        r#"'"peanut-butter-and-jelly""#
    )
}
#[test]
fn define_test_3() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"(define xyz '(x y z))"#,
        "(let ([xyz '(z y x)]) xyz)",
        r#"'(z y x)"#
    )
}
#[test]
fn define_test_4() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define doubler
            (lambda (f)
              (lambda (x) (f x x))))
        (define double (doubler +))"#,
        "(double 13)",
        r#"'26"#
    )
}
#[test]
fn define_test_5() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define doubler
            (lambda (f)
              (lambda (x) (f x x))))
        (define double-cons (doubler cons))"#,
        "(double-cons 'a)",
        r#"'(a a)"#
    )
}
#[test]
#[ignore = "this intentionaly fails by overflowing stack"]
fn define_test_6() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define doubler
            (lambda (f)
              (lambda (x) (f x x))))
        (define double-any
            (lambda (f x)
              ((doubler f) x)))"#,
        "(double-any double-any double-any) ",
        r#"'(a a)"#
    )
}
#[test]
// If you try to apply proc1 before defining proc2, you should get a undefined exception message
#[should_panic]
fn define_test_7() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define proc1
            (lambda (x y)
              (proc2 y x)))"#,
        "(proc1 'a 'b) ",
        r#"'(b a)"#
    )
}
#[test]
fn define_test_8() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define proc1
            (lambda (x y)
              (proc2 y x)))
        (define proc2 cons)"#,
        "(proc1 'a 'b) ",
        r#"'(b a)"#
    )
}
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
    assert_eval_eq_after_predefine_ast_precompute, assert_eval_eq_ast_precompute,
    ast_precompute_execute, execute_form_with_ast, execute_programm_with_ast,
};
#[test]
fn add1_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define add1
    (lambda (n)
        (+ n 1)))"#,
        r#"(add1 67)"#,
        r#"'68"#,
    )
}
#[test]
fn sub1_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define add1
    (lambda (n)
        (+ n 1)))
(define sub1
    (lambda (n)
        (- n 1)))"#,
        r#"(sub1 67)"#,
        r#"'66"#,
    )
}
#[test]
fn zero_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define add1
    (lambda (n)
        (+ n 1)))
(define sub1
    (lambda (n)
        (- n 1)))"#,
        r#"(list (zero? 67) (zero? 0) (zero? '()))"#,
        r#"'(#f #t #f)"#,
    )
}
#[test]
fn manual_plus_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define add1
    (lambda (n)
        (+ n 1)))
(define sub1
    (lambda (n)
        (- n 1)))
(define o+
    (lambda (a b)
        (cond 
            ((zero? b) a)
            (else (add1 (o+ a (sub1 b)))))))
(define o-
    (lambda (a b)
        (cond 
            ((zero? b) a)
            (else (sub1 (o- a (sub1 b)))))))
(define addtup
    (lambda (tup)
        (cond
            ((null? tup) 0)
            "#,
        r#"(o- 30 39)"#,
        r#"'-9"#,
    )
}

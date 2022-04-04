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
// Note: and in qoute still function and
#[test]
fn lat_function_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define lat?
            (lambda (l)
            (cond
                ((null? l) #t)
                ((atom? (car l)) (lat? (cdr l)))
                (else #f))))"#,
        r#"(lat? '("bacon" "and" "eggs"))"#,
        "'#t",
    )
}
#[test]
fn lat_function_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define lat?
            (lambda (l)
            (cond
                ((null? l) #t)
                ((atom? (car l)) (lat? (cdr l)))
                (else #f))))"#,
        r#"(lat? '(bacon ("and" eggs)))"#,
        "'#f",
    )
}
#[test]
fn or_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define l1 '())
        (define l2 '(d e f g))"#,
        r#"(or (null? l1) (atom? l2))"#,
        "'#t",
    )
}
#[test]
fn or_test_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
        (define l1 '(a b c))
        (define l2 '())"#,
        r#"(or (null? l1) (null? l2))"#,
        "'#t",
    )
}
#[test]
fn member_question_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define member?
    (lambda (a lat)
        (cond
            ((null? lat) #f )
            (else (or (eq? (car lat) a)
                (member? a (cdr lat)))))))"#,
        r#"(member? 'a '(fried eggs "and" scrambled eggs))"#,
        "'#f",
    )
}
#[test]
fn member_question_test_2() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define member?
    (lambda (a lat)
        (cond
            ((null? lat) #f )
            (else (or (eq? (car lat) a)
                (member? a (cdr lat)))))))"#,
        r#"(member? 'tea '(coffee tea "or" milk))"#,
        "'#t",
    )
}
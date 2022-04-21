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
// https://mvanier.livejournal.com/2897.html
#[test]
fn strict_ycombinator_test_1() {
    assert_eval_eq_after_predefine_ast_precompute(
        r#"
(define almost-factorial
    (lambda (f)
        (lambda (n)
        (cond 
            ((eq? n 0) 1)
            (else (* n (f (- n 1))))))))

(define Y 
    (lambda (f)
        ((lambda (x) (x x))
        (lambda (x) (f (lambda (y) ((x x) y)))))))

(define factorial (Y almost-factorial))"#,
        r#"(factorial 5)"#,
        r#"'120"#,
    )
}

  
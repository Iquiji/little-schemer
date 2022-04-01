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
    assert_eval_eq_ast_precompute, ast_precompute_execute, execute_form_with_ast,
    execute_programm_with_ast,
};

#[test]
fn let_test_1() {
    assert_eval_eq_ast_precompute(
        "
    (let ((list1 '(a b c)) (list2 '(d e f)))
        (cons (cons (car list1)
                    (car list2))
                (cons (car (cdr list1))
                    (car (cdr list2)))))",
        "'((a d) b e)",
    )
}
#[test]
fn let_test_2() {
    assert_eval_eq_ast_precompute(
        "
        (let ((f +) (x 2) (y 3))
        (f x y))",
        "'5",
    )
}
#[test]
fn let_test_3() {
    assert_eval_eq_ast_precompute(
        "
    (let ((x 1))
        (let ((x (+ x 1)))
            (+ x x)))",
        "'4",
    )
}
#[test]
fn let_test_4() {
    assert_eval_eq_ast_precompute(
        "
        (let ((x '((a b) c)))
        (cons (let ((x (cdr x)))
                (car x))
              (let ((x (car x)))
                (cons (let ((x (cdr x)))
                        (car x))
                      (cons (let ((x (car x)))
                              x)
                            (cdr x))))))",
        "
        (let ((x '((a b) c)))
        (cons (let ((x2 (cdr x)))
                (car x2))
              (let ((x3 (car x)))
                (cons (let ((x4 (cdr x3)))
                        (car x4))
                      (cons (let ((x5 (car x3)))
                              x5)
                            (cdr x3))))))",
    )
}

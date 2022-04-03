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
    execute_programm_with_ast,
};

#[test]
fn lambda_test_1() {
    assert_eval_eq_ast_precompute(
        "
        (let ([double-cons (lambda (x) (cons x x))])
        (double-cons 'a))",
        "'(a a)",
    )
}
#[test]
fn lambda_test_2() {
    assert_eval_eq_ast_precompute(
        "
        (let ([double-any (lambda (f x) (f x x))])
        (list (double-any + 13)
        (double-any cons 'a))) ",
        "'(26 (a a))",
    )
}
#[test]
fn lambda_test_3() {
    assert_eval_eq_ast_precompute(
        "
(let ([x 'a])
    (let ([f (lambda (y) (list x y))])
        (f 'b)
    )
)",
        "'(a b)",
    )
}
// DONE:"Currently failing! look into: https://www.scheme.com/tspl4/start.html#./start:h5
// the same bindings that were in effect when the procedure was created are in effect again when the procedure is applied.
#[test]
fn lambda_test_4() {
    assert_eval_eq_ast_precompute(
        "
(let ([f (let ([x 'sam]) (lambda (y z) (list x y z)))])
(f 'i 'am))",
        "'(sam i am)",
    )
}
#[test]
fn lambda_test_5() {
    assert_eval_eq_ast_precompute(
        "
(let ([f (let ([x 'sam])
        (lambda (y z) (list x y z)))])
(let ([x 'not-sam])
 (f 'i 'am)))",
        "'(sam i am)",
    )
}
#[test]
fn lambda_test_6() {
    assert_eval_eq_ast_precompute(
        "(let ([x 'a]) (cons x x))",
        "((lambda (x) (cons x x)) 'a)",
    )
}

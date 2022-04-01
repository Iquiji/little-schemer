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

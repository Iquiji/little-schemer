#![allow(unused_imports)]
use little_schemer::{split_whitespace_not_in_parantheses, ExpressionTypes};
use little_schemer::AtomTypes::{Bool, Integer, String};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil};
use little_schemer::FunctionTypes::{CustomFunction, InBuildFunction};
use little_schemer::Interpreter;

pub fn assert_eval_eq(a: &str, b: &str) {
    let mut interpreter_a = Interpreter::new();
    let mut interpreter_b = Interpreter::new();

    assert_eq!(interpreter_a.eval(a), interpreter_b.eval(b));
}

pub fn assert_eval_eq_ast_precompute(a: &str, b: &str) {

    let result_a = ast_precompute_execute(a);
    let result_b = ast_precompute_execute(b);

    assert_eq!(result_a,result_b);
}

fn ast_precompute_execute(i: &str) -> ExpressionTypes{
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute_on_ast(&interpreter.generate_abstract_syntax_tree(i));
    result
}
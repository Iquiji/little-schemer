#![allow(unused_imports)]
#![allow(dead_code)]
use little_schemer::helper_functions::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
use little_schemer::AtomTypes::{Bool, Integer, String};
use little_schemer::ExpressionTypes::{self, Atom, Function, List, Nil};
use little_schemer::FunctionTypes::{CustomFunction, InBuildFunction};
use little_schemer::Interpreter;

pub fn assert_eval_eq_ast_precompute(a: &str, b: &str) {
    let result_a = execute_form_with_ast(a);
    let result_b = execute_form_with_ast(b);

    assert_eq!(result_a, result_b);
}

pub fn ast_precompute_execute(i: &str) -> ExpressionTypes {
    let mut interpreter = Interpreter::new();

    interpreter.execute_on_ast(&interpreter.generate_abstract_syntax_tree(i))
}
// Programm: Form Form ...
// Return type: Vec or not Vec?
pub fn execute_programm_with_ast(programm: &str) -> Vec<ExpressionTypes> {
    let mut interpreter = Interpreter::new();

    let ast = &interpreter.generate_abstract_syntax_tree(programm);

    let mut result_vec = vec![];

    for form in ast {
        if let List(only_form) = form {
            result_vec.push(interpreter.execute_on_ast(only_form))
        } else {
            panic!(
                "Form can only have one item that has to be a list: {:?}",
                ast
            )
        }
    }
    result_vec
}

// Form: (...)
pub fn execute_form_with_ast(form: &str) -> ExpressionTypes {
    let mut interpreter = Interpreter::new();

    let ast = &interpreter.generate_abstract_syntax_tree(form);

    if let List(only_form) = &ast[0] {
        interpreter.execute_on_ast(only_form)
    } else {
        panic!(
            "Form can only have one item that has to be a list: {:?}",
            ast
        )
    }
}

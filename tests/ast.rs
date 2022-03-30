#![allow(unused_imports)]
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{self, Atom, Function, List, Nil, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
mod common;
use common::assert_eval_eq;

#[test]
fn ast_simple() {
    let interpreter = Interpreter::new();

    // Check Simple tokenizeing of "Variables"
    let programm: &str = "bibendum morbi non quam (nec dui luctus (a b (arbitrary parenthesis level) c)) rutrum nulla";
    let result = interpreter.generate_abstract_syntax_tree(programm);

    assert_eq!(
        result,
        vec![
            Variable("bibendum".to_string()),
            Variable("morbi".to_string()),
            Variable("non".to_string()),
            Variable("quam".to_string()),
            List(vec![
                Variable("nec".to_string()),
                Variable("dui".to_string()),
                Variable("luctus".to_string()),
                List(vec![
                    Variable("a".to_string()),
                    Variable("b".to_string()),
                    List(vec![
                        Variable("arbitrary".to_string()),
                        Variable("parenthesis".to_string()),
                        Variable("level".to_string())
                    ]),
                    Variable("c".to_string())
                ])
            ]),
            Variable("rutrum".to_string()),
            Variable("nulla".to_string())
        ]
    );

    let program = "((car (list atom? null? eq?)) 'a)";
    let result = interpreter.generate_abstract_syntax_tree(program);

    assert_eq!(
        result,
        vec![
            List(vec![
                List(vec![
                    Function(FunctionTypes::InBuildFunction((
                        "car".to_string(),
                        std::sync::Arc::new(little_schemer::built_ins::car),
                        1
                    ))),
                    List(vec![
                        Function(FunctionTypes::InBuildFunction((
                            "list".to_string(),
                            std::sync::Arc::new(little_schemer::built_ins::list),
                            -1
                        ))),
                        Function(FunctionTypes::InBuildFunction((
                            "atom?".to_string(),
                            std::sync::Arc::new(little_schemer::built_ins::is_atom),
                            1
                        ))),
                        Function(FunctionTypes::InBuildFunction((
                            "null?".to_string(),
                            std::sync::Arc::new(little_schemer::built_ins::is_null_list),
                            1
                        ))),
                        Function(FunctionTypes::InBuildFunction((
                            "eq?".to_string(),
                            std::sync::Arc::new(little_schemer::built_ins::are_eq),
                            1
                        ))),
                    ])
                ]),
                List(vec![
                    Variable("quote".to_string()),
                    Variable("a".to_string())
                ])
            ])
        ]
    );
}

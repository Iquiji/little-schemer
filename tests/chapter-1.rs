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
fn quoting_replacement_1() {
    let programm: &str = "bibendum morbi non quam (nec dui luctus (a b (arbitrary parenthesis level) c)) rutrum nulla";

    let result = split_whitespace_not_in_parantheses_advanced_to_quote(programm);

    assert_eq!(
        result,
        vec![
            "bibendum",
            "morbi",
            "non",
            "quam",
            "(nec dui luctus (a b (arbitrary parenthesis level) c))",
            "rutrum",
            "nulla"
        ]
    );
}

#[test]
fn quoting_replacement_2() {
    let programm: &str = "bibendum 'morbi non quam '(nec dui 'luctus ('a b '(arbitrary parenthesis 'level) c)) rutrum nulla";

    let result = split_whitespace_not_in_parantheses_advanced_to_quote(programm);

    assert_eq!(
        result,
        vec![
            "bibendum", 
            "(quote morbi)", 
            "non",
            "quam", 
            "(quote (nec dui (quote luctus) ((quote a) b (quote (arbitrary parenthesis (quote level))) c)))",
            "rutrum",
            "nulla"
        ]
    );
}

#[test]
fn eval_keyword_is_atom() {
    let programm: &str = "atom?";

    let interpreter = Interpreter::new();

    let result = interpreter.tokenizer(programm);

    match result {
        Function(func) => match func {
            InBuildFunction(func) => assert_eq!("atom?", func.0),
            CustomFunction => panic!(),
        },
        _ => panic!(),
    }
}

#[test]
fn is_atom_with_atom() {
    let programm: &str = r#"(atom? 'xd)"#;

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Atom(Bool(true)));
}
#[test]
fn is_atom_with_list() {
    let programm: &str = "(atom? '(xd))";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Atom(Bool(false)));
}
#[test]
fn empty_list() {
    let programm: &str = "'()";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, List(vec![]));
}
#[test]
fn parse_string() {
    let programm: &str = r#"'"parse_me_baby""#;

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Atom(String("parse_me_baby".to_string())));
}
#[test]
fn parse_string_number() {
    let programm: &str = "'1337";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Atom(Integer(1337)));
}

#[test]
fn split_whitespace_not_in_parantheses_test_1() {
    let programm: &str = "bibendum morbi non quam (nec dui luctus (a b (arbitrary parenthesis level) c)) rutrum nulla";

    let result = split_whitespace_not_in_parantheses(programm);

    assert_eq!(
        result,
        vec![
            "bibendum",
            "morbi",
            "non",
            "quam",
            "(nec dui luctus (a b (arbitrary parenthesis level) c))",
            "rutrum",
            "nulla"
        ]
    );
}
#[test]
fn split_whitespace_not_in_parantheses_test_2() {
    let programm: &str = "";

    let result = split_whitespace_not_in_parantheses(programm);

    assert_eq!(result, vec![""]);
}
#[test]
fn split_whitespace_not_in_parantheses_test_3() {
    let programm: &str = "xyz abc d (a) xd '(ads ad zss) () '() no";

    let result = split_whitespace_not_in_parantheses(programm);

    assert_eq!(
        result,
        vec![
            "xyz",
            "abc",
            "d",
            "(a)",
            "xd",
            "'(ads ad zss)",
            "()",
            "'()",
            "no"
        ]
    );
}

#[test]
fn car_valid_list() {
    let programm: &str = r#"(car '("a" "b" "c" 'd 'e 'f 'g 'h 'i 'j 'k))"#;

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Atom(String("a".to_string())));
}

#[test]
fn car_valid_list_2() {
    let programm: &str = r#"(car '(("a" "b" "c") 'x 'y 'z))"#;

    let result = execute_form_with_ast(programm);

    assert_eq!(
        result,
        List(vec![
            Atom(String("a".to_string())),
            Atom(String("b".to_string())),
            Atom(String("c".to_string()))
        ])
    );
}

#[test]
fn car_empty_list() {
    let programm: &str = "(car '())";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Nil);
}

#[test]
fn parse_list_extended() {
    let programm: &str = "'(() () () ())";

    let result = execute_form_with_ast(programm);

    assert_eq!(
        result,
        List(vec![List(vec![]), List(vec![]), List(vec![]), List(vec![])])
    );
}

#[test]
fn car_valid_list_3() {
    assert_eval_eq_ast_precompute(
        r#"(car '((("hotdogs")) ("and") ("pickle") "relish"))"#,
        r#"'(("hotdogs"))"#,
    )
}

#[test]
fn car_valid_list_4() {
    let programm: &str = r#"(car (car '((("hotdogs")) ("and"))))"#;

    let result = execute_form_with_ast(programm);

    assert_eq!(result, List(vec![Atom(String("hotdogs".to_string()))]));
}

#[test]
fn cdr_valid_list() {
    let programm: &str = r#"(cdr '(a b c))"#;

    let result = execute_form_with_ast(programm);

    assert_eq!(result, execute_form_with_ast("'(b c)"));
}
#[test]
fn cdr_valid_list_2() {
    let programm: &str = "(cdr '(('a 'b 'c) 'x 'y 'z))";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, execute_form_with_ast("'('x 'y 'z)"));
}
#[test]
fn cdr_valid_list_3() {
    let programm: &str = "(cdr '('hamburger))";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, execute_form_with_ast("'()"));
}

#[test]
fn cdr_empty_list() {
    let programm: &str = "(cdr '())";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, Nil);
}

/// Cant work because cant execute on 'b
#[test]
#[should_panic]
fn car_valid_list_5() {
    let programm: &str = "((car '(b c)))";

    let result = execute_form_with_ast(programm);

    assert_eq!(result, execute_form_with_ast("'b"));
}

#[test]
fn cons_empty_list() {
    let programm: &str = "(cons 'a 'a)";

    let result = execute_form_with_ast(programm);

    assert_eq!(
        result,
        List(vec![
            Atom(Symbol("a".to_string())),
            Atom(Symbol("a".to_string()))
        ])
    );
}

#[test]
fn cons_valid_1() {
    assert_eval_eq_ast_precompute("(cons 'b '(a c))", "'(b a c)");
}

#[test]
fn cons_valid_2() {
    assert_eval_eq_ast_precompute("(cons '(a b (c)) '())", "'((a b (c)))");
}

#[test]
fn cons_valid_3() {
    assert_eval_eq_ast_precompute("(cons 'a '())", "'(a)");
}

#[test]
fn kosta_test() {
    // Cannot arbitarily wrap in Parantheses to get list
    // Would need to use list? for that
    assert_eval_eq_ast_precompute("(list (car '(a b)))", "'(a)");
    assert_eval_eq_ast_precompute("(car '(a b))", "'a");
}

#[test]
fn cons_car_1() {
    assert_eval_eq_ast_precompute("(cons 'a (car '((b) c d)))", "'(a b)")
}

#[test]
fn cons_cdr_1() {
    assert_eval_eq_ast_precompute("(cons 'a (cdr '((b) c d)))", "'(a c d)")
}
#[test]
fn is_null_1() {
    assert_eval_eq_ast_precompute("(null? '())", "'#t")
}

#[test]
fn is_null_2() {
    assert_eval_eq_ast_precompute("(null? '(a b c))", "'#f")
}

#[test]
fn is_null_3() {
    assert_eval_eq_ast_precompute("(null? 'a)", "'#f");
}

#[test]
fn are_eq_test_1() {
    assert_eval_eq_ast_precompute("(eq? 'a (car '(a b c)))", "'#t")
}

#[test]
fn are_eq_test_2() {
    assert_eval_eq_ast_precompute("(eq? 'a (cdr '(a b c)))", "'#f")
}

#[test]
fn list_test_1() {
    assert_eval_eq_ast_precompute("(list 'a 'b 'c 'd)", "'(a b c d)")
}

#[test]
fn list_test_2() {
    assert_eval_eq_ast_precompute("(list (car (list 'a 'c 'd)) 'b 'c 'd)", "'(a b c d)")
}
#[test]
fn list_test_3() {
    let programm: &str = "(list (car (list 'a 'c 'd)) 'd)";

    let result = execute_form_with_ast(programm);

    assert_eq!(
        result,
        List(vec![
            Atom(Symbol("a".to_string())),
            Atom(Symbol("d".to_string()))
        ])
    );
}
#[test]
fn list_test_4_anti_test() {
    let programm: &str = "'(car '(list 'a 'c 'd) 'd)";

    let result = execute_form_with_ast(programm);

    assert_eq!(
        result,
        List(vec![
            Function(FunctionTypes::InBuildFunction((
                "car".to_string(),
                std::sync::Arc::new(little_schemer::built_ins::car),
                1
            ))),
            List(vec![
                Syntactic(Quote),
                List(vec![
                    Function(FunctionTypes::InBuildFunction((
                        "list".to_string(),
                        std::sync::Arc::new(little_schemer::built_ins::list),
                        -1
                    ))),
                    List(vec![Syntactic(Quote), Atom(Symbol("a".to_string()))]),
                    List(vec![Syntactic(Quote), Atom(Symbol("c".to_string()))]),
                    List(vec![Syntactic(Quote), Atom(Symbol("d".to_string()))])
                ])
            ]),
            List(vec![Syntactic(Quote), Atom(Symbol("d".to_string()))])
        ])
    );
}

#[test]
fn atom_test_x() {
    assert_eval_eq_ast_precompute("(atom? (quote ()))", "'#f")
}

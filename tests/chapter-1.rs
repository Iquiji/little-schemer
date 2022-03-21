#![allow(unused_imports)]
use little_schemer::AtomTypes::{Bool, Integer, String};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil};
use little_schemer::FunctionTypes::{CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
mod common;
use common::assert_eval_eq;

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

    let result = interpreter.eval_keyword(programm, true);

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
    let programm: &str = "(atom? 'xd)";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Atom(Bool(true)));
}
#[test]
fn is_atom_with_list() {
    let programm: &str = "(atom? '(xd))";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Atom(Bool(false)));
}
#[test]
fn empty_list() {
    let programm: &str = "'()";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, List(vec![]));
}
#[test]
fn parse_string() {
    let programm: &str = r#""parse_me_baby""#;

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Atom(String("parse_me_baby".to_string())));
}
#[test]
fn parse_string_number() {
    let programm: &str = "'1337";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

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

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Atom(String("a".to_string())));
}

#[test]
fn car_valid_list_2() {
    let programm: &str = r#"(car '(("a" "b" "c") 'x 'y 'z))"#;

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

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

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Nil);
}

#[test]
fn parse_list_extended() {
    let programm: &str = "'(() () () ())";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(
        result,
        List(vec![List(vec![]), List(vec![]), List(vec![]), List(vec![])])
    );
}

#[test]
fn car_valid_list_3() {
    assert_eval_eq(
        r#"(car '((("hotdogs")) ("and") ("pickle") "relish"))"#,
        r#"'(("hotdogs"))"#,
    )
}

#[test]
fn car_valid_list_4() {
    let programm: &str = r#"(car (car '((("hotdogs")) ("and"))))"#;

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, List(vec![Atom(String("hotdogs".to_string()))]));
}

#[test]
fn cdr_valid_list() {
    let programm: &str = r#"(cdr '(a b c))"#;

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, interpreter.eval("'(b c)"));
}
#[test]
fn cdr_valid_list_2() {
    let programm: &str = "(cdr '(('a 'b 'c) 'x 'y 'z))";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, interpreter.eval("'('x 'y 'z)"));
}
#[test]
fn cdr_valid_list_3() {
    let programm: &str = "(cdr '('hamburger))";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, interpreter.eval("'()"));
}

#[test]
fn cdr_empty_list() {
    let programm: &str = "(cdr '())";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Nil);
}

#[test]
fn car_valid_list_5() {
    let programm: &str = "((car '(b c)))";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, interpreter.eval("'b"));
}

#[test]
fn cons_empty_list() {
    let programm: &str = "(cons 'a 'a)";

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(programm);

    assert_eq!(result, Nil);
}

#[test]
fn cons_valid_1() {
    assert_eval_eq("(cons 'b '(a c))", "'(b a c)");
}

#[test]
fn cons_valid_2() {
    assert_eval_eq("(cons '(a b (c)) '())", "'((a b (c)))");
}

#[test]
fn cons_valid_3() {
    assert_eval_eq("(cons 'a '())", "'(a)");
}

#[test]
fn kosta_test() {
    // Cannot arbitarily wrap in Parantheses to get list
    // Would need to use list? for that
    assert_eval_eq("(list (car '(a b)))", "'(a)");
    assert_eval_eq("(car '(a b))", "'a");
}

#[test]
fn cons_car_1() {
    assert_eval_eq("(cons 'a (car '((b) c d)))", "'(a b)")
}

#[test]
fn cons_cdr_1() {
    assert_eval_eq("(cons 'a (cdr '((b) c d)))", "'(a c d)")
}
#[test]
fn is_null_1() {
    assert_eval_eq("(null? '())", "#t")
}

#[test]
fn is_null_2() {
    assert_eval_eq("(null? ('a 'b 'c))", "#f")
}

#[test]
fn is_null_3() {
    assert_eval_eq("(null? 'a)", "#f")
}

#[test]
fn display_back_to_data_programm_1() {
    let interpreted = Interpreter::new()
        .eval("(cons 'a (cdr '((b) c d)))")
        .to_string();

    assert_eq!(interpreted, "'(a c d)");
}
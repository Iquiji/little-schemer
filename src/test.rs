#![allow(unused_imports)]
use super::split_whitespace_not_in_parantheses;
use super::AtomTypes::{Bool, Integer, String};
use super::ExpressionTypes::{Atom, Function, List, Nil};
use super::FunctionTypes::{CustomFunction, InBuildFunction};
use super::Interpreter;

fn assert_eval_eq(a: &str, b: &str) {
    let interpreter_a = Interpreter::new();
    let interpreter_b = Interpreter::new();

    assert_eq!(interpreter_a.eval_part(a), interpreter_b.eval_part(b));
}

#[test]
fn eval_keyword_is_atom() {
    let programm: &str = "atom?";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_keyword(programm);

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
    let programm: &str = "atom? 'xd";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Atom(Bool(true)));
}
#[test]
fn is_atom_with_list() {
    let programm: &str = "atom? ('xd)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Atom(Bool(false)));
}
#[test]
fn empty_list() {
    let programm: &str = "'()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, List(vec![List(vec![])]));
}
#[test]
fn parse_string() {
    let programm: &str = "'parse_me_baby";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(
        result,
        List(vec![Atom(String("parse_me_baby".to_string()))])
    );
}
#[test]
fn parse_string_number() {
    let programm: &str = "'1337";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, List(vec![Atom(Integer(1337))]));
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
    let programm: &str = "car ('a 'b 'c 'd 'e 'f 'g 'h 'i 'j 'k)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Atom(String("a".to_string())));
}

#[test]
fn car_valid_list_2() {
    let programm: &str = "car (('a 'b 'c) 'x 'y 'z)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

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
    let programm: &str = "car ()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Nil);
}

#[test]
fn parse_list_extended() {
    let programm: &str = "'() '() '() '()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(
        result,
        List(vec![List(vec![]), List(vec![]), List(vec![]), List(vec![])])
    );
}

#[test]
fn car_valid_list_3() {
    assert_eval_eq(
        "(car ((('hotdogs)) ('and) ('pickle) 'relish))",
        "(('hotdogs))",
    )
}

#[test]
fn car_valid_list_4() {
    let programm: &str = "car (car ((('hotdogs)) ('and)))";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, List(vec![Atom(String("hotdogs".to_string()))]));
}

#[test]
fn cdr_valid_list() {
    let programm: &str = "(cdr ('a 'b 'c))";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, interpreter.eval_part("('b 'c)"));
}
#[test]
fn cdr_valid_list_2() {
    let programm: &str = "(cdr (('a 'b 'c) 'x 'y 'z))";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, interpreter.eval_part("('x 'y 'z)"));
}
#[test]
fn cdr_valid_list_3() {
    let programm: &str = "cdr ('hamburger)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, interpreter.eval_part(""));
}

#[test]
fn cdr_empty_list() {
    let programm: &str = "cdr ()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Nil);
}

#[test]
fn car_valid_list_5() {
    let programm: &str = "('a (car ('b 'c)) 'd)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, interpreter.eval_part("('a 'b 'd)"));
}

#[test]
fn cons_empty_list() {
    let programm: &str = "cons 'a 'a";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result, Nil);
}

#[test]
fn cons_valid_1() {
    assert_eval_eq("(cons 'b ('a 'c))", "('b 'a 'c)");
}

#[test]
fn cons_valid_2() {
    assert_eval_eq("(cons ('a 'b ('c)) '())", "(('a 'b ('c)))");
}

#[test]
fn cons_valid_3() {
    assert_eval_eq("(cons 'a '())", "('a)");
}

#[test]
fn kosta_test() {
    assert_eval_eq("((car ('a 'b)))", "('a)");
    assert_eval_eq("(car ('a 'b))", "'a");
}

#[test]
fn cons_car_1() {
    assert_eval_eq("(cons 'a (car (('b) 'c 'd)))", "('a 'b)")
}

#[test]
fn cons_cdr_1() {
    assert_eval_eq("(cons 'a (cdr (('b) 'c 'd)))", "('a 'c 'd)")
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

#![allow(unused_imports)]
use super::split_whitespace_not_in_parantheses;
use super::ExpressionTypes::{Atom,List,Function,Nil};
use super::FunctionTypes::{InBuildFunction,CustomFunction};
use super::AtomTypes::{Bool,String,Integer};
use super::Interpreter;

#[test]
fn eval_keyword_is_atom() {
    let programm: &str = "atom?";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_keyword(programm);


    match result {
        Function(func) => {
            match func {
                InBuildFunction(func) => assert_eq!("atom?",func.0) ,
                CustomFunction => panic!(),
            }
        },
        _ => panic!(),
    }
}

#[test]
fn is_atom_with_atom() {
    let programm: &str = "atom? 'xd";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,Atom(Bool(true)));
}
#[test]
fn is_atom_with_list() {
    let programm: &str = "atom? ('xd)";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,Atom(Bool(false)));
}
#[test]
fn empty_list() {
    let programm: &str = "'()";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![List(vec![])]));
}
#[test]
fn parse_string() {
    let programm: &str = "'parse_me_baby";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![Atom(String("parse_me_baby".to_string()))]));
}
#[test]
fn parse_string_number() {
    let programm: &str = "'1337";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![Atom(Integer(1337))]));
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

    assert_eq!(result,Atom(String("a".to_string())));
}

#[test]
fn car_valid_list_2() {
    let programm: &str = "car (('a 'b 'c) 'x 'y 'z)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![Atom(String("a".to_string())), Atom(String("b".to_string())), Atom(String("c".to_string()))]));
}

#[test]
fn car_empty_list() {
    let programm: &str = "car ()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result,Nil);
}

#[test]
fn parse_list_extended() {
    let programm: &str = "'() '() '() '()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result,List(vec![List(vec![]), List(vec![]), List(vec![]), List(vec![])]));
}

#[test]
fn car_valid_list_3() {
    let programm: &str = "car ((('hotdogs)) ('and) ('pickle) 'relish)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![List(vec![Atom(String("hotdogs".to_string()))])]));
}

#[test]
fn car_valid_list_4() {
    let programm: &str = "car (car ((('hotdogs)) ('and)))";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,List(vec![Atom(String("hotdogs".to_string()))]));
}

#[test]
fn cdr_valid_list() {
    let programm: &str = "cdr ('a 'b 'c)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,interpreter.eval_part("'b 'c"));
}
#[test]
fn cdr_valid_list_2() {
    let programm: &str = "cdr (('a 'b 'c) 'x 'y 'z)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,interpreter.eval_part("('x 'y 'z)"));
}
#[test]
fn cdr_valid_list_3() {
    let programm: &str = "cdr ('hamburger)";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,interpreter.eval_part(""));
}

#[test]
fn cdr_empty_list() {
    let programm: &str = "cdr ()";

    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);

    assert_eq!(result,Nil);
}
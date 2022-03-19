#![allow(unused_imports)]
use super::*;


#[test]
fn eval_keyword_is_atom() {
    let programm: &str = "atom?";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_keyword(programm);


    match result {
        ExpressionTypes::Function(func) => {
            match func {
                FunctionTypes::InBuildFunction(func) => assert_eq!("atom?",func.0) ,
                FunctionTypes::CustomFunction => panic!(),
            }
        },
        _ => panic!(),
    }
}

#[test]
fn is_atom_with_atom() {
    let programm: &str = "(atom? 'xd)";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,ExpressionTypes::Atom(AtomTypes::Bool(true)));
}
#[test]
fn is_atom_with_list() {
    let programm: &str = "(atom? ('xd))";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,ExpressionTypes::Atom(AtomTypes::Bool(false)));
}
#[test]
fn empty_list() {
    let programm: &str = r#"()"#;


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,ExpressionTypes::List(vec![]));
}
#[test]
fn parse_string() {
    let programm: &str = "'parse_me_baby";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,ExpressionTypes::Atom(AtomTypes::String("parse_me_baby".to_string())));
}
#[test]
fn parse_string_number() {
    let programm: &str = "'1337";


    let interpreter = Interpreter::new();

    let result = interpreter.eval_part(programm);


    assert_eq!(result,ExpressionTypes::Atom(AtomTypes::Integer(1337)));
}
#[test]
fn split_whitespace_not_in_parantheses_test_1() {
    let programm: &str = "bibendum morbi non quam (nec dui luctus (a b (arbitrary parenthesis level) c)) rutrum nulla";

    let result = split_whitespace_not_in_parantheses(programm);

    assert_eq!(result,vec!["".to_string()]);
}

// #[test]
// fn car_valid_list() {
//     let programm: &str = "(car ('a 'b 'c 'd 'e 'f 'g 'h 'i 'j 'k))";


//     let interpreter = Interpreter::new();

//     let result = interpreter.eval_part(programm);


//     assert_eq!(result,ExpressionTypes::Atom(AtomTypes::Integer(1337)));
// }

use super::{AtomTypes, ExpressionTypes};

/// Takes 1 arg, name: atom?
pub fn is_atom(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Atom(AtomTypes::Bool(false));
    }
    match input[0] {
        ExpressionTypes::Atom(_) => ExpressionTypes::Atom(AtomTypes::Bool(true)),
        _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
    }
}

/// Takes 1 arg, name: null?
pub fn is_null_list(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Atom(AtomTypes::Bool(false));
    }
    match &input[0] {
        ExpressionTypes::List(list) => ExpressionTypes::Atom(AtomTypes::Bool(list.is_empty())),
        _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
    }
}

/// Takes 1 arg, name: car
pub fn car(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Atom(AtomTypes::Bool(false));
    }
    match &input[0] {
        ExpressionTypes::List(list) => {
            if list.is_empty() {
                ExpressionTypes::Nil
            } else {
                list[0].clone()
            }
        }
        _ => {
            println!("!!!!! Asking for car of Something not List");
            ExpressionTypes::Nil
        }
    }
}

/// Takes 1 arg, name: cdr
pub fn cdr(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Nil;
    }
    match &input[0] {
        ExpressionTypes::List(list) => {
            if list.is_empty() {
                ExpressionTypes::Nil
            } else {
                ExpressionTypes::List(list[1..].to_vec())
            }
        }
        _ => {
            println!("!!!!! Asking for cdr of Something not List");
            ExpressionTypes::Nil
        }
    }
}

/// Takes 2 arg, name: cons
pub fn cons(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 2 {
        return ExpressionTypes::Nil;
    }
    match &input[1] {
        ExpressionTypes::List(list) => {
            let mut result = vec![input[0].clone()];
            result.extend_from_slice(list);
            ExpressionTypes::List(result)
        }
        _ => {
            println!("!!!!! Asking for cons to Something not List");
            ExpressionTypes::Nil
        }
    }
}

/// Takes x args, name: list
pub fn list(input: &[ExpressionTypes]) -> ExpressionTypes {
    ExpressionTypes::List(input.to_vec())
}

/// Takes 2 args, name: eq?
pub fn are_eq(input: &[ExpressionTypes]) -> ExpressionTypes {
    ExpressionTypes::Atom(AtomTypes::Bool(input[0].eq(&input[1])))
}

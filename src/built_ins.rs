use crate::{AtomTypes, ExpressionTypes};

/// Takes 1 arg, name: atom?
pub fn is_atom(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1{
        return ExpressionTypes::Atom(AtomTypes::Bool(false));
    }
    match input[0] {
        ExpressionTypes::Atom(_) => ExpressionTypes::Atom(AtomTypes::Bool(true)),
        _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
    }
}

/// Takes 1 arg, name: car
pub fn car(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1{
        return ExpressionTypes::Atom(AtomTypes::Bool(false));
    }
    match &input[0] {
        ExpressionTypes::List(list) => {
            if list.is_empty(){
                ExpressionTypes::Nil
            }else{
                list[0].clone()
            }
        },
        _ => {
            println!("!!!!! Asking for car of Something not List");
            ExpressionTypes::Nil
        },
    }
}

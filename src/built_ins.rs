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

/// Takes 1 arg, name: zero?
pub fn is_zero(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Nil;
    }
    match &input[0] {
        ExpressionTypes::Atom(atom) => match atom {
            AtomTypes::Integer(int) => ExpressionTypes::Atom(AtomTypes::Bool(int.eq(&0))),
            _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
        },
        _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
    }
}

/// Takes 1 arg, name: number?
pub fn is_number(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.len() != 1 {
        return ExpressionTypes::Nil;
    }
    match &input[0] {
        ExpressionTypes::Atom(atom) => match atom {
            AtomTypes::Integer(int) => ExpressionTypes::Atom(AtomTypes::Bool(true)),
            _ => ExpressionTypes::Atom(AtomTypes::Bool(false)),
        },
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
            eprintln!("!!!!! Asking for cons to Something not List");
            ExpressionTypes::List(vec![input[0].clone(), input[1].clone()])
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

/// Takes x args, name: +
pub fn number_plus(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.is_empty() {
        panic!("number_plus needs at least one argument!");
    }
    let mut temp_value = 0;
    for num in input {
        if let ExpressionTypes::Atom(atom) = num {
            if let AtomTypes::Integer(number) = atom {
                temp_value += number;
            } else {
                panic!("Atom in addition needs to be an Integer!");
            }
        } else {
            panic!("Expression in addition needs to be an Atom/Integer!");
        }
    }
    ExpressionTypes::Atom(AtomTypes::Integer(temp_value))
}

/// Takes x args, name: -
pub fn number_minus(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.is_empty() {
        panic!("number_plus needs at least one argument!");
    }
    let mut first = true;
    let mut temp_value = 0;
    for num in input {
        if let ExpressionTypes::Atom(atom) = num {
            if let AtomTypes::Integer(number) = atom {
                if first {
                    temp_value += number;
                    first = false;
                } else {
                    temp_value -= number;
                }
            } else {
                panic!("Atom in addition needs to be an Integer!");
            }
        } else {
            panic!("Expression in addition needs to be an Atom/Integer!");
        }
    }
    ExpressionTypes::Atom(AtomTypes::Integer(temp_value))
}

/// Takes x args, name: and
/// https://www.scheme.com/tspl4/control.html#./control:h0
pub fn and(input: &[ExpressionTypes]) -> ExpressionTypes {
    if input.is_empty() {
        return ExpressionTypes::Atom(AtomTypes::Bool(true));
    }
    // go over all but last one
    for num in &input[0..input.len().saturating_sub(2)] {
        if let ExpressionTypes::Atom(atom) = num {
            if let AtomTypes::Bool(bool) = atom {
                if !bool {
                    return ExpressionTypes::Atom(AtomTypes::Bool(false));
                }
            } else {
                //
            }
        } else {
            //
        }
    }
    // return last one if it doesnt evaluate to bool
    if let ExpressionTypes::Atom(atom) = &input[(input.len() - 1)] {
        if let AtomTypes::Bool(bool) = atom {
            if !bool {
                return ExpressionTypes::Atom(AtomTypes::Bool(false));
            }
        } else {
            return input[(input.len() - 1)].clone();
        }
    } else {
        return input[(input.len() - 1)].clone();
    }
    ExpressionTypes::Atom(AtomTypes::Bool(true))
}
/// Takes x args, name: or
/// https://www.scheme.com/tspl4/control.html#./control:h0
pub fn or(input: &[ExpressionTypes]) -> ExpressionTypes {
    // go over all and return if something does not evaluate to false
    for element in input {
        if let ExpressionTypes::Atom(atom) = element {
            if let AtomTypes::Bool(bool) = atom {
                if *bool {
                    return ExpressionTypes::Atom(AtomTypes::Bool(true));
                }
            } else {
                return element.clone();
            }
        } else {
            return element.clone();
        }
    }
    ExpressionTypes::Atom(AtomTypes::Bool(false))
}

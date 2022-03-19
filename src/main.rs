use core::fmt;
use std::sync::Arc;

mod built_ins;

#[cfg(test)]
mod test;

fn main() {
    let programm: String = "(atom? ('xd 'hh))".to_owned();
    //let programm: String = r#"(atom?)"#.to_owned();

    let interpreter: Interpreter = Interpreter::new();

    println!("end result: {:?}",interpreter.eval_part(&programm));
}

struct Interpreter {
    /// (Name,Function,Arg_Count)
    functions: Vec<FunctionTypes>,
}
impl Interpreter {
    fn new() -> Self {
        Interpreter {
            functions: vec![
                FunctionTypes::InBuildFunction(
                    (
                    "atom?".to_owned(),
                    Arc::new(built_ins::is_atom),
                    1,
                    )
                ),
                FunctionTypes::InBuildFunction(
                    (
                    "car".to_owned(),
                    Arc::new(built_ins::car),
                    1,
                    )
                ),
            ],
        }
    }
    fn eval_keyword(&self, word: &str) -> ExpressionTypes {
        if word.starts_with('\'') {
            let temp = &(*word).strip_prefix('\'').unwrap();

            if let Ok(int) = temp.parse::<i64>() {
                return ExpressionTypes::Atom(AtomTypes::Integer(int));
            } else {
                return ExpressionTypes::Atom(AtomTypes::String(temp.to_string()));
            }
        }
        if word == "#f" {
            return ExpressionTypes::Atom(AtomTypes::Bool(false));
        }
        if word == "#t" {
            return ExpressionTypes::Atom(AtomTypes::Bool(true));
        }

        for temp_enum in &self.functions {
            match temp_enum {
                FunctionTypes::InBuildFunction(func_tuple) => {
                    if word == func_tuple.0 {
                        return ExpressionTypes::Function(FunctionTypes::InBuildFunction(
                            func_tuple.clone(),
                        ));
                    }
                }
                FunctionTypes::CustomFunction => todo!(),
            }
        }

        ExpressionTypes::Nil
    }
    // Take part of the String Evaluate it and call self with the rest and so on
    fn eval_part(&self, s: &str) -> ExpressionTypes {
        println!("!Taking!: {}",s);

        // Take to eiter ')' or ' '
        let current_part = s.split_once([')', ' ']);
        if let Some((current_part, next_part)) = current_part {
            println!("current: '{}' next: '{}'", current_part, next_part);
            if current_part.starts_with('(') {
                println!("Cutting ( and )");
                let mut current_parsed = ExpressionTypes::Nil;
                let mut next_parsed = ExpressionTypes::Nil;

                if let Some(current_part) = current_part.strip_prefix('(') {
                    if !current_part.is_empty() {
                        current_parsed = self.eval_keyword(current_part);
                        println!("current_parsed: {:?}", current_parsed);
                    }
                }
                if let Some(next_part) = next_part.strip_suffix(')') {
                    if !next_part.is_empty() {
                        next_parsed = self.eval_part(next_part);
                        println!("next part recusive: {:?}", next_parsed);
                    }
                }
                match current_parsed.clone() {
                    ExpressionTypes::Function(temp_func_enum) => {
                        if let FunctionTypes::InBuildFunction(func) = temp_func_enum {
                            let mut args = ExpressionTypes::List(vec![]);
                            
                            match next_parsed {
                                ExpressionTypes::List(next_as_list) => {
                                    args = ExpressionTypes::List(next_as_list);
                                }
                                _ => {
                                    args = ExpressionTypes::List((&[next_parsed]).to_vec());
                                }
                            }
                            let result = if let ExpressionTypes::List(args) = args.clone() {func.1(&args)}else{ExpressionTypes::Nil};
                            println!("Running Function: {:?} with {:?} getting: {:?}",func.0,args,result);
                            return result;
                        } else {
                            unimplemented!();
                        }
                    }
                    expression_not_function => match expression_not_function {
                        ExpressionTypes::List(next_as_list) => {
                            let mut return_list = vec![current_parsed];
                            return_list.extend_from_slice(&next_as_list);
                            return ExpressionTypes::List(return_list);
                        }
                        _ => {
                            if current_parsed.is_nil() && next_parsed.is_nil(){
                                return ExpressionTypes::List(vec![]);
                            }
                            else{
                                return ExpressionTypes::List(vec![current_parsed, next_parsed]);
                            }
                        }
                    },
                }
            } else {
                let mut current_parsed = ExpressionTypes::Nil;
                let mut next_parsed = ExpressionTypes::Nil;

                if !current_part.is_empty() {
                    current_parsed = self.eval_keyword(current_part);
                    println!("current_parsed: {:?}", current_parsed);
                }

                if !next_part.is_empty() {
                    next_parsed = self.eval_part(next_part);
                    println!("next part recusive: {:?}", next_parsed);
                }
                
                if !current_parsed.is_nil() && !next_parsed.is_nil(){
                    match next_parsed {
                        ExpressionTypes::List(next_as_list) => {
                            let mut return_list = vec![current_parsed];
                            return_list.extend_from_slice(&next_as_list);
                            return ExpressionTypes::List(return_list);
                        }
                        _ => {
                            return ExpressionTypes::List(vec![current_parsed, next_parsed]);
                        }
                    }
                }
                if !current_parsed.is_nil() && next_parsed.is_nil(){
                    return current_parsed;
                }

            }
        }
        // String cant be split so this is just a single word
        else{
            return self.eval_keyword(s);
        }

        ExpressionTypes::Nil
    }
}

type BuiltInFunction = Arc<fn(&[ExpressionTypes]) -> ExpressionTypes>;

#[derive(Clone)]
pub enum FunctionTypes {
    InBuildFunction((String, BuiltInFunction, usize)),
    CustomFunction,
}
impl PartialEq for FunctionTypes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InBuildFunction(l0), Self::InBuildFunction(r0)) => l0.0 == r0.0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl fmt::Debug for FunctionTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionTypes::InBuildFunction(builtin) => {
                write!(
                    f,
                    "InBuildFunction: '{}' taking {} args",
                    builtin.0, builtin.2
                )
            }
            FunctionTypes::CustomFunction => write!(f, "Unimplemented custom Function"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionTypes {
    Atom(AtomTypes),
    List(Vec<ExpressionTypes>),
    Function(FunctionTypes),
    Nil,
}
impl ExpressionTypes{
    fn is_nil(&self) -> bool{
        matches!(self, ExpressionTypes::Nil)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum AtomTypes {
    String(String),
    Integer(i64),
    Bool(bool),
}

pub fn split_whitespace_not_in_parantheses(input: &str) -> Vec<String>{

    let mut result: Vec<String> = vec![];
    let mut current_substring = String::new();

    let mut paranthesis_depth = 0;

    for current_char in input.chars(){
        
        println!("char: '{}',depth: {:?},result: {:?}", current_char, paranthesis_depth , result);

        if current_char == '('{
            paranthesis_depth += 1;
            current_substring.push(current_char);
        }
        else if current_char == ' ' && paranthesis_depth == 0{
            result.push(current_substring);
            current_substring = String::new();
        }
        else if current_char == ')'{
            paranthesis_depth -= 1;
            current_substring.push(current_char);
        }
        else{
            current_substring.push(current_char);
        }
    }
    result.push(current_substring);

    println!("result: {:?}", result);

    if paranthesis_depth != 0{
        panic!("Parantheses not balanced!")
    }

    result
}
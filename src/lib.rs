use core::fmt;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

mod built_ins;

pub struct Interpreter {
    /// (Name,Function,Arg_Count)
    functions: Vec<FunctionTypes>,
}
impl Interpreter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Interpreter {
            functions: vec![
                FunctionTypes::InBuildFunction((
                    "atom?".to_owned(),
                    Arc::new(built_ins::is_atom),
                    1,
                )),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("cdr".to_owned(), Arc::new(built_ins::cdr), 1)),
                FunctionTypes::InBuildFunction(("cons".to_owned(), Arc::new(built_ins::cons), 2)),
                FunctionTypes::InBuildFunction((
                    "null?".to_owned(),
                    Arc::new(built_ins::is_null_list),
                    1,
                )),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
            ],
        }
    }
    pub fn eval_keyword(&self, word: &str) -> ExpressionTypes {
        // Declarition of empty List '()
        if word == "'()" {
            return ExpressionTypes::List(vec![]);
        }

        // Declaration of atoms 'atom
        if word.starts_with('\'') {
            let temp = &(*word).strip_prefix('\'').unwrap();

            if let Ok(int) = temp.parse::<i64>() {
                return ExpressionTypes::Atom(AtomTypes::Integer(int));
            } else {
                return ExpressionTypes::Atom(AtomTypes::String(temp.to_string()));
            }
        }

        // True and False
        if word == "#f" {
            return ExpressionTypes::Atom(AtomTypes::Bool(false));
        }
        if word == "#t" {
            return ExpressionTypes::Atom(AtomTypes::Bool(true));
        }

        // Check for Function Names
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
        panic!("Couldn't parse single word: {:?}", word);
        //ExpressionTypes::Nil
    }

    // Take part of the String Evaluate it and call self with the rest and so on
    // Always returns List of some Type
    pub fn eval_part(&self, s: &str) -> ExpressionTypes {
        println!("!Taking!: {}", s);
        let result;

        let chunked_input = split_whitespace_not_in_parantheses(s);

        // Split into Primary and Secondary so we can check for function at the beginning
        let primary_statement = chunked_input[0].clone();
        let secondary_statements = chunked_input[1..].to_vec();

        println!(
            "Primary: {:?},Secondary: {:?}",
            primary_statement, secondary_statements
        );

        // If our primary Statement is empty return an empty list
        if primary_statement.is_empty() {
            return ExpressionTypes::List(vec![]);
        }

        // Starts with '(' then it is a new context and should be viewed anew with recursion
        if primary_statement.starts_with('(') {
            let removed_parantheses = primary_statement
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            let primary_parsed = self.eval_part(removed_parantheses);
            let mut result_list;
            // ('a 'b 'c) 'd >>-> (('a 'b 'c) 'd)
            // ((car ('a 'b)) 'c 'd) >>-> ('a 'c 'd)
            // ((car ('a 'b))) >>-> ('a)
            // (car ('a 'b)) >>-> 'a

            // Match primary_parsed whether it has a Fuction as First Element
            if let ExpressionTypes::List(primary_vec) = &primary_parsed {
                match &primary_vec[0] {
                    ExpressionTypes::Function(func_enum) => match func_enum {
                        FunctionTypes::InBuildFunction(builtin) => {
                            println!(
                                "Now Executing Function one layer out: {:?} with args: {:?}",
                                builtin.0,
                                primary_vec[1..].to_vec()
                            );
                            let func_result = builtin.1(&primary_vec[1..]);
                            println!("resulting in: {:?}", func_result);
                            result_list = vec![func_result];
                            let context_from_secondary =
                                self.secondary_string_vec_to_context_vec(&secondary_statements);
                            result_list.extend(context_from_secondary);
                        }
                        FunctionTypes::CustomFunction => todo!(),
                    },
                    _ => {
                        result_list = vec![primary_parsed];
                        let context_from_secondary =
                            self.secondary_string_vec_to_context_vec(&secondary_statements);
                        result_list.extend(context_from_secondary);
                    }
                }
            } else {
                panic!(
                    "primary parsed didnt return a List which it always should: {:?}",
                    primary_parsed
                );
            }

            result = ExpressionTypes::List(result_list);
        }
        // We do not have a potential Function call as the primary
        else {
            let parsed_primary = self.eval_keyword(&primary_statement);
            let context_from_secondary =
                self.secondary_string_vec_to_context_vec(&secondary_statements);

            // ('a ('b)) >-> 'a ('b) >-> ('a ('b))
            // (car ('b)) >-> car ('b) >-> 'b

            match parsed_primary {
                ExpressionTypes::Function(function_enum) => {
                    match function_enum {
                        // Call the function with the rest as context
                        FunctionTypes::InBuildFunction(inbuilt) => {
                            println!(
                                "returning Function: {}, context: {:?}",
                                inbuilt.0, context_from_secondary
                            );

                            // Check for context amount
                            if context_from_secondary.len() != inbuilt.2 {
                                panic!("Function has gotten more or less context than it wants");
                            }

                            let mut arm_result = vec![ExpressionTypes::Function(
                                FunctionTypes::InBuildFunction(inbuilt),
                            )];
                            arm_result.extend_from_slice(&context_from_secondary);
                            result = ExpressionTypes::List(arm_result);
                        }
                        FunctionTypes::CustomFunction => todo!(),
                    }
                }
                ExpressionTypes::Atom(atom) => {
                    let mut arm_result = vec![ExpressionTypes::Atom(atom)];
                    arm_result.extend_from_slice(&context_from_secondary);
                    result = ExpressionTypes::List(arm_result);
                }
                ExpressionTypes::Nil => todo!(),
                ExpressionTypes::List(primary_list) => {
                    let mut arm_result = vec![ExpressionTypes::List(primary_list)];
                    arm_result.extend_from_slice(&context_from_secondary);
                    result = ExpressionTypes::List(arm_result);
                }
            }
        }
        println!("Input: '{:?}' produced: {:?}", s, result);
        result
    }
    /// Returns ExpressionTypes::List from '(...) or quote (...) context
    fn list_context_eval() -> ExpressionTypes {
        unimplemented!()
    }
    /// Returns ExpressionTypes from (procedure args...) Context
    fn procedure_context_eval() -> ExpressionTypes {
        unimplemented!()
    }

    fn secondary_string_vec_to_context_vec(
        &self,
        secondary_vec: &[String],
    ) -> Vec<ExpressionTypes> {
        // Parse all Secondary as Context
        let mut context_from_secondary = vec![];
        for secondary in secondary_vec {
            // Error secondary should never be empty
            if secondary.is_empty() {
                panic!("Secondary should never be empty");
            }

            // Starts with '(' then it is a new context and should be viewed anew with recursion
            if secondary.starts_with('(') {
                let removed_parantheses = secondary
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap();

                context_from_secondary.push(self.eval_part(removed_parantheses));
            }
            // Just a normal part we can parse with eval_keyword
            else {
                context_from_secondary.push(self.eval_keyword(&secondary));
            }
        }
        context_from_secondary
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
impl Display for FunctionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            FunctionTypes::InBuildFunction(to_display) => write!(f, "{}", to_display.0),
            FunctionTypes::CustomFunction => todo!(),
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
impl ExpressionTypes {
    fn is_nil(&self) -> bool {
        matches!(self, ExpressionTypes::Nil)
    }
}
impl Display for ExpressionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            ExpressionTypes::Atom(to_display) => write!(f, "{}", to_display),
            ExpressionTypes::List(to_display) => {
                write!(f, "({})", {
                    to_display
                        .iter()
                        .map(|me| me.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                })
            }
            ExpressionTypes::Function(to_display) => write!(f, "{}", to_display),
            ExpressionTypes::Nil => write!(f, "~nil~"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AtomTypes {
    String(String),
    Integer(i64),
    Bool(bool),
}
impl Display for AtomTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            AtomTypes::String(to_display) => write!(f, "'{}", to_display),
            AtomTypes::Integer(to_display) => write!(f, "'{}", to_display),
            AtomTypes::Bool(to_display) => write!(f, "'{}", if *to_display { "#t" } else { "f" }),
        }
    }
}

pub fn split_whitespace_not_in_parantheses(input: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut current_substring = String::new();

    let mut paranthesis_depth = 0;

    for current_char in input.chars() {
        // println!(
        //     "char: '{}',depth: {:?},result: {:?}",
        //     current_char, paranthesis_depth, result
        // );

        if current_char == '(' {
            paranthesis_depth += 1;
            current_substring.push(current_char);
        } else if current_char == ' ' && paranthesis_depth == 0 {
            if !current_substring.is_empty() {
                result.push(current_substring);
            }
            current_substring = String::new();
        } else if current_char == ')' {
            paranthesis_depth -= 1;
            current_substring.push(current_char);
        } else {
            current_substring.push(current_char);
        }
    }

    result.push(current_substring);

    //println!("result: {:?}", result);

    if paranthesis_depth != 0 {
        panic!("Parantheses not balanced!")
    }

    result
}

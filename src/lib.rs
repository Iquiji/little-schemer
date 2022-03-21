use core::fmt;
use std::{
    ascii::AsciiExt,
    fmt::{Debug, Display},
    sync::Arc,
    vec,
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
                FunctionTypes::InBuildFunction(("list".to_owned(), Arc::new(built_ins::list), -1)),
                FunctionTypes::InBuildFunction((
                    "null?".to_owned(),
                    Arc::new(built_ins::is_null_list),
                    1,
                )),
                FunctionTypes::InBuildFunction((
                    "atom?".to_owned(),
                    Arc::new(built_ins::is_atom),
                    1,
                )),
                FunctionTypes::InBuildFunction(("eq?".to_owned(), Arc::new(built_ins::are_eq), 2)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
            ],
        }
    }
    /// "a" -> String("a")
    /// '"a" -> String("a")
    /// 42 -> Integer(42)
    /// a -> Variable? a
    pub fn eval_keyword(&self, word: &str, allow_variables: bool) -> ExpressionTypes {
        // Declarition of empty List '()
        if word == "'()" {
            return ExpressionTypes::List(vec![]);
        }

        // Declaration of atoms 'atom -> Symbol(atom)
        // if word.starts_with('\'') {
        //     let temp = &(*word).strip_prefix('\'').unwrap();

        //     if let Ok(int) = temp.parse::<i64>() {
        //         return ExpressionTypes::Atom(AtomTypes::Integer(int));
        //     } else {
        //         return ExpressionTypes::Atom(AtomTypes::Symbol(temp.to_string()));
        //     }
        // }

        // Double Quotes for String
        if word.starts_with('"') && word.ends_with('"') {
            return ExpressionTypes::Atom(AtomTypes::String(
                word.strip_prefix('"')
                    .unwrap()
                    .strip_suffix('"')
                    .unwrap()
                    .to_string(),
            ));
        }

        // Just number to Integer
        if let Ok(int) = word.parse::<i64>() {
            return ExpressionTypes::Atom(AtomTypes::Integer(int));
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
        if allow_variables {
            ExpressionTypes::Variable(word.to_string())
        } else {
            ExpressionTypes::Atom(AtomTypes::Symbol(word.to_string()))
        }
    }
    pub fn eval(&mut self, s: &str) -> ExpressionTypes {
        println!("Eval: {}", s);

        let chunked_input = split_whitespace_not_in_parantheses_advanced_to_quote(s);

        // Split into Primary and Secondary so we can check for function at the beginning
        let primary_statement = chunked_input[0].clone();
        let secondary_statements = chunked_input[1..].to_vec();

        println!(
            "Primary: {:?},Secondary: {:?}",
            primary_statement, secondary_statements
        );
        // Procedure Call Context
        // Or (quote atom) or (quote (...)) but we check for "syntactic procedures" inside
        if primary_statement.starts_with('(') {
            let removed_parantheses = primary_statement
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            let primary_evaluated = self.procedure_context_eval(removed_parantheses);

            if !secondary_statements.is_empty() {
                panic!("Secondaries with procedure in top level func is not supported yet");
            }
            primary_evaluated
        } else {
            #[allow(clippy::collapsible_else_if)]
            if chunked_input.len() == 1 {
                self.eval_keyword(&chunked_input[0], true)
            } else {
                unimplemented!("top level multiple inputs not supported yet?");
                //self.list_context_eval(s)
            }
        }
    }
    //: we need to differentiate between (obj1 obj2 ...) and (procedure arg ...)

    /// Returns ExpressionTypes::List from '(...) or quote (...) context
    /// '("a" "b" "c") -> List([String("a"),String("b"),String("c")])
    fn list_context_eval(&mut self, input: &str, allow_function_calls: bool) -> ExpressionTypes {
        println!("list_context_eval: {:?}", input);
        let chunked_input = split_whitespace_not_in_parantheses(input);
        let mut result_vec = vec![];

        for chunk in chunked_input {
            // Error secondary should never be empty
            if chunk.is_empty() {
                return ExpressionTypes::List(vec![]);
            }

            // Starts with '(' then it is a new context and should be viewed anew with recursion
            if chunk.starts_with('(') {
                let removed_parantheses =
                    chunk.strip_prefix('(').unwrap().strip_suffix(')').unwrap();

                result_vec.push(if allow_function_calls {
                    self.procedure_context_eval(removed_parantheses)
                } else {
                    self.list_context_eval(removed_parantheses, allow_function_calls)
                });
            }
            // Just a normal part we can parse with eval_keyword
            else {
                result_vec.push(self.eval_keyword(&chunk, false));
            }
        }

        ExpressionTypes::List(result_vec)
    }
    /// Returns called Procedure Result from (procedure arg...) Context
    fn procedure_context_eval(&mut self, input: &str) -> ExpressionTypes {
        println!("procedure_context_eval: {:?}", input);
        let chunked_input = split_whitespace_not_in_parantheses_advanced_to_quote(input);

        // Split into Primary and Secondary so we can check for function at the beginning
        let primary_statement = chunked_input[0].clone();
        let secondary_statements = chunked_input[1..].to_vec();

        // Check for still in Parantheses for primary
        if primary_statement.starts_with('(') {
            let removed_parantheses = primary_statement
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            self.procedure_context_eval(removed_parantheses)
        }
        // Check for Syntactic
        else if primary_statement == "quote" {
            let remove_quote = input.strip_prefix("quote ").unwrap();
            // Starts with '(' then it is a new list instead of atom
            if remove_quote.starts_with('(') {
                let removed_parantheses = remove_quote
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap();

                self.list_context_eval(removed_parantheses, false)
            } else {
                self.eval_keyword(remove_quote, false)
            }
        }
        // Check if Function and error otherwise because we need first to be function!
        else {
            let primary_eval = self.eval_keyword(&primary_statement, true);
            match &primary_eval {
                ExpressionTypes::Function(func_enum) => match func_enum {
                    FunctionTypes::InBuildFunction(builtin) => {
                        let context_from_secondary =
                            self.secondary_string_vec_to_context_vec(&secondary_statements, true);
                        println!(
                            "Now Executing Function one layer out: {:?} with args: {:?}",
                            builtin.0, context_from_secondary
                        );
                        // Check for context amount. If -1 Ignore because it takes an arbitrary amount of Arguments
                        if context_from_secondary.len() as i32 != builtin.2 && builtin.2 != -1 {
                            panic!("Function has gotten more or less context than it wants");
                        }
                        let func_result = builtin.1(&context_from_secondary);
                        println!("resulting in: {:?}", func_result);

                        func_result
                    }
                    FunctionTypes::CustomFunction => todo!(),
                },
                not_function => {
                    panic!(
                        "First Item of Procedure call is not a Function!: {:?}",
                        not_function
                    );
                }
            }
        }
    }

    pub fn secondary_string_vec_to_context_vec(
        &mut self,
        secondary_vec: &[String],
        allow_variables: bool,
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
                if allow_variables {
                    context_from_secondary.push(self.procedure_context_eval(removed_parantheses));
                } else {
                    context_from_secondary
                        .push(self.list_context_eval(removed_parantheses, allow_variables));
                }
            }
            // Just a normal part we can parse with eval_keyword
            else {
                context_from_secondary.push(self.eval_keyword(secondary, allow_variables));
            }
        }
        context_from_secondary
    }
}

type BuiltInFunction = Arc<fn(&[ExpressionTypes]) -> ExpressionTypes>;

#[derive(Clone)]
pub enum FunctionTypes {
    InBuildFunction((String, BuiltInFunction, i32)),
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

impl Eq for FunctionTypes {}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionTypes {
    Atom(AtomTypes),
    List(Vec<ExpressionTypes>),
    Function(FunctionTypes),
    Variable(String),
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
            ExpressionTypes::Variable(to_display) => write!(f, "{}", to_display),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtomTypes {
    String(String),
    Integer(i64),
    Bool(bool),
    Symbol(String),
}
impl Display for AtomTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            AtomTypes::Symbol(to_display) => write!(f, r#"'{}"#, to_display),
            AtomTypes::String(to_display) => write!(f, r#"'"{}""#, to_display),
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

/// 'x -> (quote x) / '(a b c) -> (quote (a b c))
pub fn split_whitespace_not_in_parantheses_advanced_to_quote(input: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut current_substring = String::new();

    let mut paranthesis_depth = 0;
    let mut quote_stack: Vec<i32> = vec![];

    for current_char in input.chars() {
        // println!(
        //     "depth: {:?},char: {:?},stack: {:?}",
        //     paranthesis_depth, current_char, quote_stack
        // );
        if current_char == '\'' {
            quote_stack.push(paranthesis_depth);
            current_substring = current_substring + "(quote ";
        } else if current_char == '(' {
            paranthesis_depth += 1;
            current_substring.push(current_char);
        } else if current_char == ' ' {
            if !quote_stack.is_empty() && quote_stack[quote_stack.len() - 1] == paranthesis_depth {
                current_substring.push(')');
                quote_stack.pop();
            }
            if paranthesis_depth == 0 {
                if !current_substring.is_empty() {
                    for _ in 0..quote_stack.len() {
                        current_substring.push(')');
                        quote_stack.pop();
                    }
                    result.push(current_substring);
                }
                current_substring = String::new();
            } else {
                current_substring.push(current_char);
            }
        } else if current_char == ')' {
            if !quote_stack.is_empty() && quote_stack[quote_stack.len() - 1] == paranthesis_depth {
                current_substring.push(')');
                quote_stack.pop();
            }
            paranthesis_depth -= 1;
            current_substring.push(current_char);
        } else {
            current_substring.push(current_char);
        }
        // println!(
        //     "depth: {:?},char: {:?},stack: {:?}",
        //     paranthesis_depth, current_char, quote_stack
        // );
    }
    for _ in 0..quote_stack.len() {
        current_substring.push(')');
        quote_stack.pop();
    }
    result.push(current_substring);

    //println!("result: {:?}", result);

    if paranthesis_depth != 0 {
        panic!("Parantheses not balanced!")
    }

    result
}

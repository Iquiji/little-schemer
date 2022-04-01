use core::fmt;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
    vec,
};

use scoping_context::Scope;

pub mod built_ins;
mod scoping_context;

pub struct Interpreter {
    /// (Name,Function,Arg_Count)
    functions: Vec<FunctionTypes>,
    scope_stack: Vec<Scope>,
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
                FunctionTypes::InBuildFunction(("eq?".to_owned(), Arc::new(built_ins::are_eq), 2)),
                FunctionTypes::InBuildFunction(("list".to_owned(), Arc::new(built_ins::list), -1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
                FunctionTypes::InBuildFunction(("car".to_owned(), Arc::new(built_ins::car), 1)),
            ],
            scope_stack: vec![Scope::new()],
        }
    }
    pub fn tokenizer(&self, word: &str) -> ExpressionTypes {
        // If it is a Syntactic keyword:
        if word == "quote" {
            return ExpressionTypes::Syntactic(SyntacticTypes::Quote);
        }
        if word == "let" {
            return ExpressionTypes::Syntactic(SyntacticTypes::Let);
        }
        // if word == "quote"{
        //     return TokenTypes::Syntactic("quote".to_string());
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
        ExpressionTypes::Variable(word.to_string())
    }
    /// Resolve Variable from interpreter scope stack
    pub fn resolve_variable(&mut self, var: &str) -> ExpressionTypes {
        println!("current stack: '{:?}'", self.scope_stack);

        for scope in self.scope_stack.iter().rev() {
            if let Some(lookuped) = scope.get(var) {
                println!("{:?} resolved to: {:?}", var, lookuped);
                return lookuped;
            }
        }
        panic!("Could not resolve Variable: {:?}", var);
    }
    /// Generate Syntax Tree without Executing Functions or Doing any checking
    pub fn generate_abstract_syntax_tree(&self, input: &str) -> Vec<ExpressionTypes> {
        let chunked_input = split_whitespace_not_in_parantheses_advanced_to_quote(input);
        let mut result_vec = vec![];

        for chunk in chunked_input {
            // Error cant have "" in chunks
            if chunk.is_empty() {
                eprintln!(
                    "cant handle empty string chunk: '{}': '{}' \n skipping for now",
                    input, chunk
                );
                continue;
            }

            // Starts with '(' then it is a level deeper and should be looked at anew:
            if chunk.starts_with('(') {
                let removed_parantheses =
                    chunk.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
                // Eval new layer with level deeper and then push that as a list to result vec:
                result_vec.push(ExpressionTypes::List(
                    self.generate_abstract_syntax_tree(removed_parantheses),
                ));
            }
            // Just a normal part we can parse with eval_keyword
            else {
                result_vec.push(self.tokenizer(&chunk));
            }
        }
        // println!("generated ast: {:#?}", result_vec);
        result_vec
    }

    // TODO:
    // Proposal: execute_or_return
    // returning execution if in list and else returning value
    fn execute_on_list_lookup_on_variable_return_else(
        &mut self,
        input: ExpressionTypes,
    ) -> ExpressionTypes {
        match &input {
            ExpressionTypes::List(list) => self.execute_on_ast(list),
            ExpressionTypes::Variable(var) => self.resolve_variable(var),
            _ => input,
        }
    }

    // Syntactic
    //  Let -> Special shit
    //  Quote -> Return inner

    // Not Syntactic
    //  Variable -> Look up to Function?
    //  List -> Execute inner to Function?

    //  Function? -> Execute on all Secondaries

    pub fn execute_on_ast(&mut self, input: &[ExpressionTypes]) -> ExpressionTypes {
        println!("execute_on_ast: {:?}", input);
        let return_result: ExpressionTypes;

        if let ExpressionTypes::Syntactic(syntactic) = &input[0] {
            // Syntactic
            //  Let -> Special shit
            //  Quote -> Return inner
            match syntactic {
                SyntacticTypes::Let => {
                    // let ([var expr]...) body1 body2... body-n
                    // Return result of body-n
                    let mut new_scope = Scope::new();
                    match &input[1] {
                        ExpressionTypes::List(binding_list) => {
                            for binding in binding_list {
                                match binding {
                                    ExpressionTypes::List(binding_pair) => {
                                        if binding_pair.len() != 2 {
                                            panic!("binding pair need to have a length of 2! instead found: {:?}",binding_pair);
                                        }
                                        if let ExpressionTypes::Variable(variable_string) = &binding_pair[0]{
                                            new_scope.insert_single((variable_string.clone(),self.execute_on_list_lookup_on_variable_return_else(binding_pair[1].clone())));
                                        }else{
                                            panic!("first element of binding pair needs to be a variable! instead found: {:?}",binding_pair[0])
                                        }
                                    },
                                    _ => panic!(
                                        "binding in binding_list of let needs to be a list! instead found: {:?}",
                                        input[1]
                                    ),
                                }
                            }
                        }
                        _ => panic!(
                            "first argument of let needs to be a list! instead found: {:?}",
                            input[1]
                        ),
                    }
                    // Push onto scope stack
                    println!("Pushing '{:?}' onto the scope stack", new_scope);
                    self.scope_stack.push(new_scope);
                    // Then execute expr until last element
                    for expr in &input[2..(input.len() - 1)] {
                        self.execute_on_list_lookup_on_variable_return_else(expr.clone());
                    }
                    // last element will be returned
                    return_result = self.execute_on_list_lookup_on_variable_return_else(
                        input[input.len() - 1].clone(),
                    );

                    println!(
                        "Popping '{:?}' from the scope stack",
                        self.scope_stack.pop()
                    );
                }
                SyntacticTypes::Quote => {
                    // The symbol hello must be quoted in order to prevent Scheme from treating hello as a variable.
                    // https://www.scheme.com/tspl4/start.html#./start:h2
                    fn replace_recursive(input: ExpressionTypes) -> ExpressionTypes {
                        match input.clone() {
                            ExpressionTypes::List(quoted_list) => {
                                let mut new_list_replaced_var_with_symbols = vec![];

                                for item in quoted_list {
                                    new_list_replaced_var_with_symbols
                                        .push(replace_recursive(item));
                                }

                                ExpressionTypes::List(new_list_replaced_var_with_symbols)
                            }
                            ExpressionTypes::Variable(var) => {
                                ExpressionTypes::Atom(AtomTypes::Symbol(var))
                            }
                            _ => input,
                        }
                    }
                    // TODO: symbols instead of variables in quoted context
                    return_result = replace_recursive(input[1].clone());
                    //return input[1].clone();
                }
            }
        } else {
            // Not Syntactic
            //  Variable -> Look up to Function?
            //  List -> Execute inner to Function?

            //  Function? -> Execute on all Secondaries
            match &input[0] {
                ExpressionTypes::Syntactic(_) => unreachable!(),
                //  Function? -> Execute on all Secondaries
                ExpressionTypes::Function(func_enum) => {
                    let mut secondaries_proccessed_vec = vec![];
                    // Preprocess Secondaries
                    for secondary_item in &input[1..] {
                        secondaries_proccessed_vec.push(
                            self.execute_on_list_lookup_on_variable_return_else(
                                secondary_item.clone(),
                            ),
                        );
                    }
                    return_result = self.execute_function_pre_parsed_secondaries(
                        func_enum.clone(),
                        &secondaries_proccessed_vec,
                    );
                }
                //  Variable -> Look up to Function?
                ExpressionTypes::Variable(var) => {
                    match self.resolve_variable(var) {
                        ExpressionTypes::Function(func_enum) => {
                            let mut secondaries_proccessed_vec = vec![];
                            // Preprocess Secondaries
                            for secondary_item in &input[1..] {
                                secondaries_proccessed_vec.push(
                                    self.execute_on_list_lookup_on_variable_return_else(
                                        secondary_item.clone(),
                                    ),
                                );
                            }
                            return_result = self.execute_function_pre_parsed_secondaries(
                                func_enum,
                                &secondaries_proccessed_vec,
                            );
                        }
                        not_a_function => panic!(
                            "Variable resolved to not a function in primary position; got: {:?}",
                            not_a_function
                        ),
                    }
                }
                //  List -> Execute inner to Function?
                ExpressionTypes::List(list) => {
                    // Execute Primary List recursively and check output type
                    match self.execute_on_ast(list) {
                        ExpressionTypes::Function(func_enum) => {
                            let mut secondaries_proccessed_vec = vec![];
                            // Preprocess Secondaries
                            for secondary_item in &input[1..] {
                                secondaries_proccessed_vec.push(
                                    self.execute_on_list_lookup_on_variable_return_else(
                                        secondary_item.clone(),
                                    ),
                                );
                            }
                            return_result = self.execute_function_pre_parsed_secondaries(
                                func_enum,
                                &secondaries_proccessed_vec,
                            );
                        }
                        not_a_function => panic!(
                            "List resolved to not a function in primary position; got: {:?}",
                            not_a_function
                        ),
                    }
                }
                // Cant Execute on not Function?
                _ => panic!(
                    "input[0] is not a variable or resolvable to a function: {:?}",
                    input[0]
                ),
            }
        }
        println!("execute_on_ast resulting in: {:?}", return_result);
        return_result
    }

    fn execute_function_pre_parsed_secondaries(
        &mut self,
        func: FunctionTypes,
        secondaries: &[ExpressionTypes],
    ) -> ExpressionTypes {
        match func {
            FunctionTypes::InBuildFunction(builtin) => {
                println!(
                    "execute_function_pre_parsed_secondaries Function: {:?} with args: {:?}",
                    builtin.0, secondaries
                );
                // Check for context amount. If -1 Ignore because it takes an arbitrary amount of Arguments
                if secondaries.len() as i32 != builtin.2 && builtin.2 != -1 {
                    panic!("Function has gotten more or less context than it wants");
                }
                let func_result = builtin.1(secondaries);
                println!("resulting in: {:?}", func_result);

                func_result
            }
            FunctionTypes::CustomFunction => todo!(),
        }
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
pub enum SyntacticTypes {
    Let,
    Quote,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionTypes {
    Syntactic(SyntacticTypes),
    Atom(AtomTypes),
    List(Vec<ExpressionTypes>),
    Function(FunctionTypes),
    Variable(String),
    Nil,
}
impl ExpressionTypes {
    fn _is_nil(&self) -> bool {
        matches!(self, ExpressionTypes::Nil)
    }
}
impl Display for ExpressionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            ExpressionTypes::Atom(to_display) => write!(f, "{}", to_display),
            ExpressionTypes::List(to_display) => {
                write!(f, "(list {})", {
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
            ExpressionTypes::Syntactic(to_display) => match to_display {
                SyntacticTypes::Let => todo!(),
                SyntacticTypes::Quote => todo!(),
            },
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
    let input = input.replace("\n", " ").replace("  ", " ");

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
            current_substring += "(quote ";
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

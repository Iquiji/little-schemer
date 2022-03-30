#![allow(unused_imports)]
use little_schemer::AtomTypes::{Bool, Integer, String, Symbol};
use little_schemer::ExpressionTypes::{Atom, Function, List, Nil, Variable};
use little_schemer::FunctionTypes::{self, CustomFunction, InBuildFunction};
use little_schemer::Interpreter;
use little_schemer::{
    split_whitespace_not_in_parantheses, split_whitespace_not_in_parantheses_advanced_to_quote,
};
mod common;
use common::assert_eval_eq;

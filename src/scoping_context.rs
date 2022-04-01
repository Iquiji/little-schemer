#[allow(unused_imports)]
use crate::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
    map: HashMap<String, ExpressionTypes>,
}
impl Scope {
    pub fn new() -> Self {
        Scope {
            map: HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn new_with(with: &[(String, ExpressionTypes)]) -> Self {
        let mut scope = Scope::new();
        scope.insert_multiple(with);
        scope
    }
    pub fn get(&self, s: &str) -> Option<ExpressionTypes> {
        self.map.get(s).cloned()
    }
    #[allow(dead_code)]
    pub fn insert_multiple(&mut self, input: &[(String, ExpressionTypes)]) {
        for pair in input {
            self.map.insert(pair.0.clone(), pair.1.clone());
        }
    }
    pub fn insert_single(&mut self, input: (String, ExpressionTypes)) {
        self.map.insert(input.0.clone(), input.1);
    }
}

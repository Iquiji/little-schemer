#[allow(unused_imports)]
use crate::*;
use std::collections::HashMap;

#[derive(Debug,Clone)]
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
    pub fn get_all(&self) -> Vec<(String,ExpressionTypes)>{
        self.map.iter().map(|x| (x.0.clone(),x.1.clone())).collect()
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
    pub fn compress(scope_list: &[Scope]) -> Scope{
        let mut compressed_scope = Scope::new();
        for scope in scope_list{
            compressed_scope.insert_multiple(&scope.get_all());
        }
        compressed_scope
    }
}

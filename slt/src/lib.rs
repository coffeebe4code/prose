use std::collections::BTreeMap;
pub struct SLT {
    table: BTreeMap<String, usize>,
}

impl SLT {
    pub fn new() -> Self {
        SLT {
            table: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, value: String, variable: usize) -> () {
        self.table.insert(value, variable);
    }
    pub fn lookup(&self, value: &String) -> Option<usize> {
        match self.table.get(value) {
            Some(v) => Some(*v),
            _ => None,
        }
    }
}

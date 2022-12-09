pub struct Var<'source> {
    symbol: &'source str,
    hash: usize,
    ids: Vec<usize>,
}

impl<'source> Var<'source> {
    pub fn new(symbol: &'source str, hash: usize, first: usize) -> Self {
        Var {
            symbol,
            hash,
            ids: vec![first],
        }
    }
    pub fn version(&mut self, next: usize) -> () {
        self.ids.push(next);
    }
    pub fn symbol_comp_last(&self, hash: usize, comp: &'source str) -> Option<usize> {
        if hash == self.hash {
            if comp.eq(self.symbol) {
                return Some(*self.ids.last().unwrap());
            }
        }
        return None;
    }
}

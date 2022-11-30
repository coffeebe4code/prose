use instr::Instr;

pub enum BlockKind {
    PlainBlock,
    IfBlock,
    RetBlock,
    RetBlockVoid,
    FirstBlock,
    TopBlock,
    LeafBlock,
}

pub struct Block<'a> {
    kind: BlockKind,
    preds: Vec<&'a Block<'a>>,
    succs: Vec<&'a Block<'a>>,
    instructions: Vec<Instr>,
    id: usize,
}

impl<'a> Block<'a> {
    pub fn new(id: usize) -> Self {
        Block {
            kind: BlockKind::PlainBlock,
            preds: vec![],
            succs: vec![],
            instructions: vec![],
            id,
        }
    }
}

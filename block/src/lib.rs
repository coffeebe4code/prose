pub enum BlockKind {
    PlainBlock,
    IfBlock,
    RetBlock,
    RetBlockVoid,
    FirstBlock,
    TopBlock,
    LeafBlock,
}

// add instructions vec
pub struct Block<'a> {
    kind: BlockKind,
    preds: Vec<&'a Block<'a>>,
    succs: Vec<&'a Block<'a>>,
    id: usize,
}

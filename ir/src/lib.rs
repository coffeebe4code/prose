use block::Block;
use gen::GenSource;

pub struct IrSource<'a> {
    reg_id: usize,
    block_id: usize,
    main_exit: usize,
    blocks: Vec<Block<'a>>,
    gen: GenSource,
}

impl<'a> IrSource<'a> {
    pub fn ir_new() -> Self {
        IrSource {
            reg_id: 0,
            block_id: 0,
            main_exit: 0,
            blocks: vec![],
            gen: GenSource::new(),
        }
    }
}

use block::Block;
use gen::GenSource;

pub struct IrSource {
    reg_id: usize,
    block_id: usize,
    main_exit: usize,
    blocks: Vec<Block>,
    gen: GenSource,
}

impl IrSource {
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

use ast::*;
use cranelift_codegen::ir::{Function, Signature, UserFuncName};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};

pub struct FIRSource {
    namespace: u32,
    ctx: FunctionBuilderContext,
    funcs: Vec<Function>,
    fname: u32,
}

impl FIRSource {
    pub fn new(namespace: u32) -> Self {
        let ctx = FunctionBuilderContext::new();
        FIRSource {
            namespace,
            ctx,
            funcs: vec![],
            fname: 0,
        }
    }
    pub fn begin(&mut self, func: &Expr) {
        let sig = Signature::new(CallConv::SystemV);
        let name = UserFuncName::user(self.namespace, self.fname);
        let mut func = Function::with_name_signature(name, sig);
        self.fname += 1;
        let mut builder = FunctionBuilder::new(&mut func, &mut self.ctx);
        let root_block = builder.create_block();
    }
}

use ast::*;
use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::ir::function::DisplayFunction;
use cranelift_codegen::ir::types::*;
use cranelift_codegen::ir::AbiParam;
use cranelift_codegen::ir::{Function, InstBuilder, Signature, UserFuncName};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings;
use cranelift_codegen::verifier::verify_function;
use cranelift_frontend::*;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use perror::*;

pub struct IRSource {
    package: u32,
    fname: u32,
    variables: usize,
}

impl IRSource {
    pub fn new(package: u32) -> Self {
        IRSource {
            package,
            fname: 0,
            variables: 0,
        }
    }
    pub fn handle_block(
        &mut self,
        op: &Block,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let temp = self.recurse(&op.exprs[0], builder).unwrap();
        Ok(temp)
    }
    pub fn handle_ret(&mut self, op: &RetOp, builder: &mut FunctionBuilder) -> ResultFir<Variable> {
        let temp = self.recurse(&op.expr, builder).unwrap();
        let arg = builder.use_var(temp);
        builder.ins().return_(&[arg]);
        Ok(temp)
    }
    pub fn handle_num(
        &mut self,
        num: &Number,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let result = Variable::new(self.variables);
        self.variables += 1;
        builder.declare_var(result, I64);
        let temp = builder
            .ins()
            .iconst(I64, num.val.slice.parse::<i64>().unwrap());
        builder.def_var(result, temp);
        Ok(result)
    }
    pub fn recurse(&mut self, expr: &Expr, builder: &mut FunctionBuilder) -> ResultFir<Variable> {
        match expr {
            Expr::Block(op) => self.handle_block(&op, builder),
            Expr::RetOp(op) => self.handle_ret(&op, builder),
            Expr::Number(op) => self.handle_num(&op, builder),
            _ => panic!("developer error unexpected expression"),
        }
    }
    pub fn begin(&mut self, func_def: FuncDef) -> Function {
        let mut ctx = FunctionBuilderContext::new();
        let mut sig = Signature::new(CallConv::SystemV);
        let name = UserFuncName::user(self.package, self.fname);
        // TODO:: put the optional vec of expr directly on funcdef
        if let Some(val) = func_def.args {
            val.iter()
                .for_each(|_x| sig.params.push(AbiParam::new(I64)));
        }
        sig.returns.push(AbiParam::new(I64));
        let mut func = Function::with_name_signature(name, sig);
        self.fname += 1;
        let mut builder = FunctionBuilder::new(&mut func, &mut ctx);
        let root_block = builder.create_block();
        builder.append_block_params_for_function_params(root_block);
        builder.switch_to_block(root_block);
        let _result = self.recurse(&func_def.block, &mut builder);
        builder.seal_block(root_block);
        builder.finalize();
        func
    }
    pub fn get_ir(self, func: &Function) -> Result<DisplayFunction> {
        let flags = settings::Flags::new(settings::builder());
        let res = verify_function(func, &flags);
        match res {
            Err(error) => panic!("get_ir: {}", error),
            _ => Ok(func.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::*;
    use token::*;
    #[test]
    fn it_should_build_ret_5() {
        let func_def = FuncDef::new(
            None,
            Lexeme {
                token: Token::Const,
                span: 0..3,
                slice: "const".to_string(),
            },
            expr!(
                Symbol,
                Lexeme {
                    token: Token::Symbol,
                    span: 4..5,
                    slice: "x".to_string()
                }
            ),
            None,
            expr!(
                Block,
                vec![expr!(
                    RetOp,
                    Lexeme {
                        token: Token::Return,
                        span: 8..10,
                        slice: "return".to_string()
                    },
                    expr!(
                        Number,
                        Lexeme {
                            token: Token::Num,
                            span: 6..7,
                            slice: "5".to_string()
                        }
                    )
                )]
            ),
        );
        let mut fir = IRSource::new(0);
        let result = fir.begin(func_def);
        /*
         * function u0:0() -> i64 system_v
         *  {
         *      block0:
         *      v0 = iconst.i64 5
         *      return v0  ; v0 = 5
         *  }
         */
        assert_eq!(format!("{}", fir.get_ir(&result).unwrap()), "function u0:0() -> i64 system_v {\nblock0:\n    v0 = iconst.i64 5\n    return v0  ; v0 = 5\n}\n");
    }
}

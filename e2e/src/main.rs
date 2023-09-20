use ast::*;
use ir::*;
use lexer::*;
use object::*;
use parser::*;
use slt::*;

fn main() {
    let lex = ProseLexer::new("pub const main = fn() { let m = 7; let x = 5; return x + m; }");
    let mut parse = Parser::new(lex);
    let ast_parsed = parse.func().unwrap();
    let mut ir = IRSource::new(0, SLT::new());
    match *ast_parsed {
        Expr::FuncDef(val) => {
            let result = ir.begin(val);
            println!("{}", ir.get_ir(&result).unwrap());
            build_main(result);
        }
        _ => panic!("not a func def!"),
    }
}

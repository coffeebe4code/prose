use gen::GenSource;
use instr::Instr;
use opcode::Op;

pub struct Vm {
    gen: GenSource,
    regs: [usize; 254],
}

impl Vm {
    pub fn new(gen: GenSource) -> Self {
        Vm {
            gen,
            regs: [0; 254],
        }
    }
    pub fn run(&mut self) -> Result<usize, usize> {
        let mut result = 0;
        loop {
            if self.gen.get_remaining() == 0 {
                break;
            }
            let instr = self.gen.read32();
            let op = Op::from32(instr[0]);
            let dst = instr[1];
            match op {
                Op::NoOp => {}
                Op::RetVal => {
                    result = self.regs[dst as usize];
                }

                _ => {
                    print!("{}", "developer error");
                }
            }
        }

        return Ok(result);
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

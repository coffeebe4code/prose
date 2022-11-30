use gen::GenSource;
use instr::Instr;
use opcode::Op;

pub struct Vm {
    gen: GenSource,
    regs: [usize; 1024],
}

impl Vm {
    pub fn new(gen: GenSource) -> Self {
        Vm {
            gen,
            regs: [0; 1024],
        }
    }
    pub fn run(&mut self) -> Result<usize, usize> {
        loop {
            if self.gen.get_remaining() == 0 {
                break;
            }
            let instr = self.gen.read32();
            let op = Op::from32(instr[0]);
            match op {
                _ => {
                    print!("{}", "developer error");
                }
            }
        }

        return Ok(self.regs[0]);
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

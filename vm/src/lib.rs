use gen::GenSource;
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
        self.gen.reset();
        let mut result = 0;
        loop {
            if self.gen.get_remaining() == 0 {
                break;
            }
            let instr = self.gen.read64_parts();
            let op = Op::from(instr[0]);
            let dst = self.gen.read64();
            eprintln!("dst {}", dst);
            match op {
                Op::F64Sub => {}
                Op::F64Const => {
                    let srcl = self.gen.read64();
                    let _srcr = self.gen.read64();
                    self.regs[dst as usize] = srcl;
                }
                Op::F64Add => {
                    let srcl = self.gen.read64();
                    let srcr = self.gen.read64();
                    binary_op!(+, self, dst, srcl, srcr);
                }
                Op::F64Mul => {}
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

#[macro_export]
macro_rules! binary_op {
    ($op:tt, $self:expr, $dst:expr, $srcl:expr, $srcr:expr) => {
        $self.regs[$dst] = ($self.regs[$srcl] as f64 $op $self.regs[$srcr] as f64) as usize;
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

pub struct Gen {
    byte_len: usize,
    binary: Vec<u8>,
}

impl Gen {
    pub fn new() -> Self {
        Gen {
            byte_len: 0,
            binary: vec![0],
        }
    }
    pub fn add8(&mut self, val: u8) -> &mut Self {
        self.binary.push(val);
        return self;
    }
    pub fn add32_safe(&mut self, first: &[u8; 4], second: &[u8; 4]) -> &mut Self {
        self.binary.extend_from_slice(first);
        self.binary.extend_from_slice(second);
        return self;
    }
    pub fn add32(&mut self, val: &[u8; 4]) -> &mut Self {
        self.binary.extend_from_slice(val);
        return self;
    }
    pub fn add64(&mut self, val: &[u8; 8]) -> &mut Self {
        self.binary.extend_from_slice(val);
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

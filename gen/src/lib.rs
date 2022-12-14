pub struct GenSource {
    pos: usize,
    binary: Vec<u8>,
}

impl GenSource {
    pub fn new() -> Self {
        GenSource {
            pos: 0,
            binary: vec![],
        }
    }
    pub fn add32(&mut self, val: [u8; 4]) -> &mut Self {
        self.binary.extend_from_slice(&val);
        self.pos += 4;
        return self;
    }
    pub fn add64(&mut self, val: [u8; 8]) -> &mut Self {
        self.binary.extend_from_slice(&val);
        self.pos += 8;
        return self;
    }
    pub fn read32<'a>(&'a mut self) -> &'a [u8] {
        let ret = &self.binary[self.pos..self.pos + 4];
        self.pos += 4;
        return ret;
    }
    pub fn read64<'a>(&'a mut self) -> &'a [u8] {
        let ret = &self.binary[self.pos..self.pos + 8];
        self.pos += 8;
        return ret;
    }
    pub fn is_64_aligned(&self) -> bool {
        (&self.binary[self.pos] as *const u8 as usize) % 8 == 0
    }
    pub fn get_len(&self) -> usize {
        return self.binary.len();
    }
    pub fn get_remaining(&self) -> usize {
        return self.binary.len() - self.pos;
    }
    pub fn reset(&mut self) -> &mut Self {
        self.pos = 0;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_insert_and_read() {
        let mut gen = GenSource::new();
        gen.add64([1u8; 8]);
        let val = gen.reset().read64();
        assert_eq!(val, &[1; 8]);
    }
    #[test]
    fn it_should_insert_with_pos() {
        let mut gen = GenSource::new();
        gen.add64([1u8; 8]);
        gen.add32([0u8; 4]);
        gen.reset();
        assert_eq!(gen.read64(), [1; 8]);
        assert_eq!(gen.get_len(), 12);
        assert_eq!(gen.get_remaining(), 4);
        assert_eq!(gen.read32(), [0; 4]);
    }
}

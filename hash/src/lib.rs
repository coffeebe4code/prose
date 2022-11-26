#[cfg(target_pointer_width = "64")]
static OFFSET_HASH_BASE: usize = 14695981039346656037;
#[cfg(target_pointer_width = "32")]
static OFFSET_HASH_BASE: usize = 2166136261;

pub fn hash(value: &str) -> usize {
    let mut result: usize = 0;
    let mut chunks = value.as_bytes().chunks_exact(4);
    for c in &mut chunks {
        let bytes = u32::from_ne_bytes(c.try_into().unwrap()) as usize;
        result = (bytes ^ result).wrapping_mul(OFFSET_HASH_BASE);
    }
    if chunks.remainder().len() > 0 {
        for b in chunks.remainder().iter() {
            result = ((*b as usize) | result).wrapping_mul(OFFSET_HASH_BASE);
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hashes() {
        let test = "one";
        let result = hash(test);
        assert_eq!(1603589979668169483, result);
    }
}

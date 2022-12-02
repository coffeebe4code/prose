#[cfg(target_pointer_width = "64")]
static OFFSET_HASH_BASE: usize = 14695981039346656037;

pub fn hash(value: &str) -> usize {
    let mut result: usize = 0;
    let chunks = value.as_bytes();
    for c in chunks {
        result = ((*c as usize) ^ result).wrapping_mul(OFFSET_HASH_BASE);
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
        assert_eq!(12997758714278952556, result);
    }
}

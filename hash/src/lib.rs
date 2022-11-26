static OFFSET_HASH_BASE: usize = 2166136261;

pub fn hash(value: &str) -> usize {
    let mut result: usize = 0;
    for c in value.chars().into_iter() {
        result = ((c as usize) ^ result) * OFFSET_HASH_BASE;
    }
    return result;
}

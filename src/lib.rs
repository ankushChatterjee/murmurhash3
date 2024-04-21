const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;
const C3: u32 = 0xe6546b64;
const C4: u32 = 0x85ebca6b;
const C5: u32 = 0xc2b2ae35;
const R1: u32 = 15;
const R2: u32 = 13;
const R3: u32 = 16;

pub fn murmur_hash(key: &[u8], seed: u32) -> u32 {
    let mut hash = seed;

    key.chunks_exact(4).for_each(|chunk| {
        let mut k = u32::from_le_bytes(chunk.try_into().unwrap());
        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = hash.rotate_left(R2);
        hash = hash.wrapping_mul(5).wrapping_add(C3);
    });

    let remainder = key.chunks_exact(4).remainder();
    if remainder.len() != 0 {
        // process remainder for murmurhash3
        let mut k = 0;
        let mut i = 0;
        for byte in remainder {
            k ^= ((*byte) as u32) << (8 * i);
            i += 1;
        }
        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);
        hash ^= k;
    }
    hash ^= key.len() as u32;

    // mix step for murmurhash3
    hash ^= hash >> R3;
    hash = hash.wrapping_mul(C4);
    hash ^= hash >> R2;
    hash = hash.wrapping_mul(C5);
    hash ^= hash >> R3;

    return hash;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_murmur() {
        let key1 = murmur_hash("eva01".as_bytes(), 7521);
        let key2 = murmur_hash("eva01".as_bytes(), 7521);

        println!("key1: {}", key1);
        println!("key2: {}", key2);
    }
}

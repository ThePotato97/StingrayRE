use std::convert::TryInto;

pub fn stingray_hash(data: &[u8]) -> u64 {
    const SEED: u64 = 0;
    const MIX: u64 = 0xC6A4A7935BD1E995;
    const SHIFTS: u32 = 47;

    let length = data.len();
    let mut hash = SEED ^ (length as u64).wrapping_mul(MIX);

    // Process full 64-bit chunks
    for chunk in data.chunks(8) {
        if chunk.len() == 8 {
            let mut key = u64::from_le_bytes(chunk.try_into().unwrap());

            key = key.wrapping_mul(MIX);
            key ^= key >> SHIFTS;
            key = key.wrapping_mul(MIX);

            hash ^= key;
            hash = hash.wrapping_mul(MIX);
        } else {
            // Process remaining bytes
            for (i, &byte) in chunk.iter().enumerate() {
                hash ^= (byte as u64) << (i * 8);
            }
        }
    }

    hash = hash.wrapping_mul(MIX);
    hash ^= hash >> SHIFTS;
    hash = hash.wrapping_mul(MIX);
    hash ^= hash >> SHIFTS;
    hash
}
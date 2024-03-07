/*
 * Custom hashers for map/sets
 * this is faster that default hasher.. but want to look into simd/maybe highway hasher
 */


use std::hash::BuildHasherDefault;
use std::hash::Hasher;

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

// this supports string and numbers..
pub struct GeneralFnvHasher {
    pub(crate) hash: u64,
}

impl Hasher for GeneralFnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64;
            self.hash = self.hash.wrapping_mul(FNV_PRIME);
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

impl Default for GeneralFnvHasher {
    fn default() -> Self {
        GeneralFnvHasher {
            hash: FNV_OFFSET_BASIS,
        }
    }
}

pub type GeneralHasher = BuildHasherDefault<GeneralFnvHasher>;

// this is just for u32
pub struct U32FnvHasher {
    pub(crate) hash: u64,
}

impl Hasher for U32FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.hash ^= u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64;
        self.hash = self.hash.wrapping_mul(FNV_PRIME);
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

impl Default for U32FnvHasher {
    fn default() -> Self {
        U32FnvHasher {
            hash: FNV_OFFSET_BASIS,
        }
    }
}

pub type U32Hasher = BuildHasherDefault<U32FnvHasher>;

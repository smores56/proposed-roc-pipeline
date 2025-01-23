use std::collections::HashMap;

use crate::soa::Index;

#[derive(Debug, Default)]
pub struct DedupedStringStore {
    indices_by_hash: HashMap<u32, usize>,
    strings: Vec<String>,
}

impl DedupedStringStore {
    pub fn insert(&mut self, s: &str) -> DedupedStringId {
        let hash = fnv_str_hash(s);
        let index = match self.indices_by_hash.get(&hash) {
            Some(str_index) => Index::new(*str_index as u32),
            None => {
                let len = self.strings.len();
                self.strings.push(s.to_owned());
                self.indices_by_hash.insert(hash, len);

                Index::new(len as u32)
            }
        };

        DedupedStringId(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DedupedStringId(Index<String>);

impl core::ops::Index<DedupedStringId> for DedupedStringStore {
    type Output = str;

    fn index(&self, index: DedupedStringId) -> &Self::Output {
        &self.strings[index.0.index()]
    }
}

/// A simple string hash.
///
/// http://isthe.com/chongo/tech/comp/fnv/#FNV-1
pub fn fnv_str_hash(s: &str) -> u32 {
    const FNV_PRIME_32_BIT: u32 = 16777619;
    const OFFSET_BASIS_32_BIT: u32 = 2166136261;

    let mut hash = OFFSET_BASIS_32_BIT;

    for byte in s.bytes() {
        hash *= FNV_PRIME_32_BIT;
        hash ^= byte as u32;
    }

    hash
}

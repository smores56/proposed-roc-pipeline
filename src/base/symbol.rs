use std::collections::HashMap;

use super::{
    ident::{Ident, IdentAttributes, IdentProblems},
    module::ModuleId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentId(u32);

impl IdentId {
    fn new_with_attributes(id: u32, _attributes: IdentAttributes) -> Self {
        // TODO: embed attributes into start of id
        Self(id)
    }

    fn effectful(&self) -> bool {
        todo!()
    }

    fn ignored(&self) -> bool {
        todo!()
    }

    fn reassignable(&self) -> bool {
        todo!()
    }

    fn uppercase(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    module_id: ModuleId,
    ident_id: IdentId,
}

#[derive(Debug, Default, Clone)]
pub struct SymbolStore {
    // TODO: this is half-baked, figure out how this should be structured
    ident_ids_per_module: HashMap<u32, HashMap<ModuleId, Vec<IdentId>>>,
    next_ident_id_per_module: HashMap<ModuleId, u32>,
    text_index_per_symbol: HashMap<Symbol, u64>,
    problems_per_text_hash: HashMap<u32, IdentProblems>,
    texts: Vec<String>,
}

impl SymbolStore {
    fn get_next_ident_id(&mut self, module_id: ModuleId) -> IdentId {
        let next_ref = self.next_ident_id_per_module.entry(module_id).or_insert(0);
        let ident_id = IdentId(*next_ref);
        *next_ref += 1;

        ident_id
    }

    pub fn insert_new(&mut self, module_id: ModuleId, ident: Ident) -> Symbol {
        let text_hash = fnv_str_hash(ident.get_raw_text());
        let ident_id = self.get_next_ident_id(module_id);

        let ident_ids_for_module = self
            .ident_ids_per_module
            .entry(text_hash)
            .or_default()
            .entry(module_id)
            .or_default();
        ident_ids_for_module.push(ident_id);

        let symbol = Symbol {
            module_id,
            ident_id,
        };

        // self.attributes_per_s
        //     .insert(symbol, ident.attributes());
        self.problems_per_text_hash
            .insert(text_hash, ident.problems());

        // TODO: ingest data into these
        // text_index_per_symbol: HashMap<Symbol, u64>,
        // texts: Vec<String>,

        symbol
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

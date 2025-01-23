use crate::{
    base::{
        problem::{CompilerProblem, Problem},
        string_store::DedupedStringId,
        symbol::SymbolStore,
        type_var::TypeVar,
    },
    soa::{Index, Slice},
};

#[derive(Default)]
pub struct Env {
    //     utable: UnificationTable,
    pub variables: Vec<TypeVar>,
    //     pub variable_slices: Vec<VariableSubsSlice>,
    //     pub tuple_elem_indices: Vec<usize>,
    //     pub record_fields: Vec<RecordField<()>>,
    //     pub variable_slices: Vec<VariableSubsSlice>,
    //     pub tag_name_cache: TagNameCache,
    pub symbols: SymbolStore,
    // no deduping because these tend to be unique and potentially large
    string_literals: Vec<String>,
    // field_names: FieldNameCache,
    tag_names: TagNameCache,
    problems: Vec<Problem>,
    compiler_problems: Vec<CompilerProblem>,
}

impl Env {
    pub fn add_string_literal(&mut self, s: String) -> StringLiteralId {
        let len = self.string_literals.len();
        self.string_literals.push(s);

        StringLiteralId(Index::new(len as u32))
    }

    pub fn add_field_name(&mut self, s: &str) -> FieldNameId {
        FieldNameId(self.field_names.insert(s))
    }

    pub fn add_tag_name(&mut self, s: &str) -> TagNameId {
        TagNameId(self.tag_names.insert(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringLiteralId(Index<String>);

impl core::ops::Index<StringLiteralId> for Env {
    type Output = str;

    fn index(&self, index: StringLiteralId) -> &Self::Output {
        &self.string_literals[index.0.index()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldNameId(DedupedStringId);

impl core::ops::Index<FieldNameId> for Env {
    type Output = str;

    fn index(&self, index: FieldNameId) -> &Self::Output {
        &self.field_names[index.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagNameId(DedupedStringId);

impl core::ops::Index<TagNameId> for Env {
    type Output = str;

    fn index(&self, index: TagNameId) -> &Self::Output {
        &self.tag_names[index.0]
    }
}

#[derive(Debug, Clone, Default)]
pub struct TagNameCache {
    tag_names: Vec<String>,
    tag_names_slices: Vec<Slice<String>>,
}

impl TagNameCache {
    pub fn get_mut(&mut self, tag_name: &TagName) -> Option<&mut SubsSlice<TagName>> {
        match self.tag_names.iter().position(|u| u == tag_name) {
            Some(index) => Some(&mut self.tag_names_slices[index]),
            None => None,
        }
    }

    pub fn push(&mut self, tag_name: &TagName, slice: SubsSlice<TagName>) {
        self.tag_names.push(tag_name.clone());
        self.tag_names_slices.push(slice);
    }
}

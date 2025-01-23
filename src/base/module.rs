#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(u32);

pub struct ModuleStore {
    pub ids: Vec<u32>,
    pub names: Vec<String>,
}

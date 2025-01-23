#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeVar(u32);

#[derive(Debug, Default)]
struct TypeVarStore {
    next_var_id: u32,
}

impl TypeVarStore {
    pub fn fresh(&mut self) -> TypeVar {
        let next_var = self.next_var_id;
        self.next_var_id += 1;

        TypeVar(next_var)
    }
}

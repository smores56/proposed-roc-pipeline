#[derive(Debug)]
pub enum Problem {
    CompilerProblem(CompilerProblem),
}

#[derive(Debug)]
pub enum CompilerProblem {
    SpecializeTypes(SpecializeTypesProblem),
    LiftFunctions(LiftFunctionsProblem),
    SolveFunctions(SolveFunctionsProblem),
    SpecializeFunctions(SpecializeFunctionsProblem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializeTypesProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiftFunctionsProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolveFunctionsProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializeFunctionsProblem {}

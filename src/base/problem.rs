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
    LowerIr(LowerIrProblem),
    ReferenceCount(ReferenceCountProblem),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecializeTypesProblem {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiftFunctionsProblem {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveFunctionsProblem {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecializeFunctionsProblem {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LowerIrProblem {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceCountProblem {}

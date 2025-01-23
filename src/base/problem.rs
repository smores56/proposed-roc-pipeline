#[derive(Debug)]
pub enum Problem {}

#[derive(Debug)]
pub enum CompilerProblem {
    SpecializeTypes(SpecializeTypesProblem),
    FunctionLift(FunctionLiftProblem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecializeTypesProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionLiftProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSolveProblem {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionSpecializeProblem {}

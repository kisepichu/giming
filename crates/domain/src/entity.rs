use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub code: String,
    pub title: String,
    pub statement: String,
    pub constraints: Vec<String>,
    pub input_format: String,
    pub samples: Vec<Sample>,

    pub point: usize,
    pub time_limit: usize,
    pub memory_limit: usize,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemSummary {
    pub id: String,
    pub code: String,
}

#[derive(Serialize, Deserialize)]
pub struct SamplePath {
    pub input: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct Argument {
    pub ty: String,
    pub var: String,
}

#[derive(Serialize, Deserialize)]
pub struct IOSpec {
    pub arguments: Vec<Argument>,
    pub input_part: String,
    pub test_input_part: String,
    pub sample_paths: Vec<SamplePath>,
}

#[derive(Serialize)]
pub struct WorkProblem<'p> {
    pub problem: &'p Problem,
    pub io_spec: IOSpec,
}

#[derive(Serialize)]
pub struct Workspace<'p> {
    pub contest_id: String,
    pub work_problems: Vec<WorkProblem<'p>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

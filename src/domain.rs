use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContestInProblemContext {
    pub url: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemContext {
    pub contest: ContestInProblemContext,
    pub alphabet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Test {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub url: String,
    pub name: String,
    pub context: ProblemContext,
    pub memory_limit: usize,
    pub time_limit: usize,

    pub tests: Option<Vec<Test>>,
    pub available_languages: Option<Vec<usize>>,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contest {
    pub url: String,
    pub name: String,
    pub problems: Vec<Problem>,
    pub start_date: Option<String>,
}

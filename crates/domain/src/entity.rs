#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct ProblemSummary {
    pub id: String,
    pub code: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sample {
    pub input: String,
    pub output: String,
}

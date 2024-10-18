use domain::entity::{Prediction, Problem, WorkProblem};

pub struct Predictor {}

impl Predictor {
    pub fn predict(problem: &Problem) -> domain::entity::WorkProblem {
        WorkProblem {
            problem,
            solutions: vec![],
            prediction: Prediction {
                arguments: None,
                input_part: None,
                test_input_part: None,
                sample_paths: vec![],
            },
        } // todo
    }
}

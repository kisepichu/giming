use domain::entity::{Argument, IOSpec, Problem, SamplePath};

pub struct IOInferrer {}

impl IOInferrer {
    pub fn infer(_problem: &Problem) -> IOSpec {
        IOSpec {
            arguments: vec![
                Argument {
                    ty: "usize".to_string(),
                    var: "n".to_string(),
                },
                Argument {
                    ty: "Vec<usize>".to_string(),
                    var: "a".to_string(),
                },
                Argument {
                    ty: "String".to_string(),
                    var: "s".to_string(),
                },
            ],
            input_part: r#"input! {
        n: usize,
        a: [usize; n],
        s: String,
    }"#
            .to_string(),
            test_input_part: r#"input! {
            from source,
            n: usize,
            a: [usize; n],
            s: String,
        }"#
            .to_string(),
            sample_paths: vec![
                SamplePath {
                    input: "testcases/a/in/0.in".to_string(),
                    output: "testcases/a/out/0.out".to_string(),
                },
                SamplePath {
                    input: "testcases/a/in/1.in".to_string(),
                    output: "testcases/a/out/1.out".to_string(),
                },
                SamplePath {
                    input: "testcases/a/in/2.in".to_string(),
                    output: "testcases/a/out/2.out".to_string(),
                },
            ],
        } // todo

        // IOSpec {
        //     arguments: vec![],
        //     input_part: "".to_string(),
        //     test_input_part: "".to_string(),
        //     sample_paths: vec![],
        // } // todo
    }
}

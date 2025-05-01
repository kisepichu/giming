use domain::entity::{Argument, IOSpec, Problem};

pub struct IOInferrer {}

impl IOInferrer {
    pub fn infer(_problem: &Problem) -> IOSpec {
        // todo
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
    }
"#
            .to_string(),
            test_input_part: r#"input! {
            from source,
            n: usize,
            a: [usize; n],
            s: String,
        }
"#
            .to_string(),
        } // todo

        // IOSpec {
        //     arguments: vec![],
        //     input_part: "".to_string(),
        //     test_input_part: "".to_string(),
        // } // todo
    }
}

use domain::entity::{Argument, IOSpec, Problem};

pub struct IOInferrer {}

impl IOInferrer {
    pub fn infer(_problem: &Problem) -> IOSpec {
        // todo
        IOSpec {
            arguments: vec![
                Argument {
                    ty: "isize".to_string(),
                    var: "n".to_string(),
                },
                Argument {
                    ty: "isize".to_string(),
                    var: "c".to_string(),
                },
                Argument {
                    ty: "Vec<isize>".to_string(),
                    var: "a".to_string(),
                },
            ],
            input_part: r#"input! {
        n: isize,
        c: isize,
        a: [isize; n],
    }
"#
            .to_string(),
            test_input_part: r#"input! {
            from source,
            n: isize,
            c: isize,
            a: [isize; n],
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

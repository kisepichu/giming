use std::{
    fmt::{self},
    io::{stdout, Write},
};

use proconio::*;

macro_rules! out {
    ($w:expr;) => {
        {
            writeln!($w).unwrap();
            Ok(()) as anyhow::Result<()>
        }
    };
    ($w:expr; v $head:expr) => {
        {
            writeln!($w, "{}", SliceDisplay(&($head))).unwrap();
            Ok(()) as anyhow::Result<()>
        }
    };
    ($w:expr; $head:expr) => {
        {
            writeln!($w, "{}", &($head)).unwrap();
            Ok(()) as anyhow::Result<()>
        }
    };
    ($w:expr; v $head:expr, $($tail:expr),*) => {
        {
            write!($w, "{} ", SliceDisplay(&($head))).unwrap();
            out!($w; $($tail ),*);
            Ok(()) as anyhow::Result<()>
        }
    };
    ($w:expr; $head:expr, $($tail:expr),*) => {
        {
            write!($w, "{} ", &($head)).unwrap();
            out!($w; $($tail),*);
            Ok(()) as anyhow::Result<()>
        }
    };
}

{% if prediction_success %}
{% else %}
// prediction failed

{% endif %}
fn solve<W: Write>(io: &mut W{% for a in formal_arguments %}, {{ a }}{% endfor %}) -> anyhow::Result<()> {
    
}

#[allow(unused)]
struct SliceDisplay<'a, T: 'a>(&'a [T]);

impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, " {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}

fn main() {
    {{ input_part }}
    let mut stdout = stdout().lock();
    solve(&mut stdout{% for a in actual_arguments %}, {{ a }}{% endfor %}).unwrap();
}

#[cfg(test)]
mod test {
    use proconio::{source::once::OnceSource, *};

    #[rstest::rstest(input, expected,
        {% for s in sample_paths %}
        case(include_str!("../../{{ s.input }}"), include_str!("../../{{ s.output }}")),
        {% endfor %}
    )]
    fn test_solve(input: &str, expected: &str) {
        let source = OnceSource::from(input);
        {{ test_input_part }}
        let mut buf = Vec::new();
        super::solve(&mut buf, n, s).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(expected.to_string(), output);
    }
}

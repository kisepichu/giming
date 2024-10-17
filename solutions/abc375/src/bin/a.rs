use std::{
    fmt::{self},
    io::{stdout, Write},
};

use proconio::*;

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

macro_rules! out {
    ($w:expr;) => {
        writeln!($w).unwrap();
    };
    ($w:expr; v $head:expr) => {
        writeln!($w, "{}", SliceDisplay(&($head))).unwrap();
    };
    ($w:expr; $head:expr) => {
        writeln!($w, "{}", &($head)).unwrap();
    };
    ($w:expr; v $head:expr, $($tail:expr),*) => {
        write!($w, "{} ", SliceDisplay(&($head))).unwrap();
        out!($w; $($tail ),*);
    };
    ($w:expr; $head:expr, $($tail:expr),*) => {
        write!($w, "{} ", &($head)).unwrap();
        out!($w; $($tail),*);
    };
}

fn solve<W: Write>(w: &mut W, _n: usize, s: Vec<char>) {
    out!(w; s.windows(3).filter(|v| v == &['#', '.', '#']).count());
}

fn main() {
    input! {
        n: usize,
        s: marker::Chars
    };
    let mut stdout = stdout().lock();
    solve(&mut stdout, n, s);
}

#[cfg(test)]
mod test {
    use proconio::{source::once::OnceSource, *};

    #[rstest::rstest(input, expected,
        case(include_str!("../../testcases/a/in/0.in"), include_str!("../../testcases/a/out/0.out")),
        case(include_str!("../../testcases/a/in/1.in"), include_str!("../../testcases/a/out/1.out")),
        case(include_str!("../../testcases/a/in/2.in"), include_str!("../../testcases/a/out/2.out")),
    )]
    fn test_solve(input: &str, expected: &str) {
        let source = OnceSource::from(input);
        input! {
            from source,
            n: usize,
            s: marker::Chars
        }
        let mut buf = Vec::new();
        super::solve(&mut buf, n, s);
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(expected.to_string(), output);
    }
}

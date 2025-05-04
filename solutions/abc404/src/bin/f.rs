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


fn solve<W: Write>(w: &mut W, n: isize, c: isize, a: Vec<isize>) -> anyhow::Result<()> {
    Ok(())
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
    input! {
        n: isize,
        c: isize,
        a: [isize; n],
    }
    let mut stdout = stdout().lock();
    solve(&mut stdout, n, c, a).unwrap();
}

#[cfg(test)]
mod test {
    use proconio::{source::once::OnceSource, *};

    #[rstest::rstest(input, expected,
        case(r"3 2 2 3
", r"0.222222222222222
"),
        case(r"10 1 10 1
", r"1.000000000000000
"),
        case(r"100 10 10 2
", r"0.401263060761621
"),
        
    )]
    fn test_solve(input: &str, expected: &str) {
        let source = OnceSource::from(input);
        input! {
            from source,
            n: isize,
            c: isize,
            a: [isize; n],
        }

        let mut buf = Vec::new();
        super::solve(&mut buf, n, c, a).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(expected.to_string(), output);
    }
}


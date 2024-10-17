use std::fmt;

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
    () => {
        println!();
    };
    (v $head:expr) => {
        println!("{}", SliceDisplay(&($head)));
    };
    ($head:expr) => {
        println!("{}", &($head));
    };
    (v $head:expr, $($tail:expr),*) => {
        print!("{} ", SliceDisplay(&($head)));
        out!($($tail ),*);
    };
    ($head:expr, $($tail:expr),*) => {
        print!("{} ", &($head));
        out!($($tail),*);
    };
}

fn main() {
    let a = vec![1, 2, 3];
    let s = "asdf".to_string();
    let x = 3;

    out!(v a, s, x);
}

use std::{
    fmt::{self},
    io::{Write, stdout},
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

fn main() -> anyhow::Result<()> {
    Ok(())
}

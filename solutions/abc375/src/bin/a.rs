use proconio::*;
use std::io::{stdout, Write};

fn solve<W: Write>(w: &mut W, _n: usize, s: Vec<char>) {
    writeln!(
        w,
        "{}",
        s.windows(3).filter(|v| v == &['#', '.', '#']).count()
    )
    .unwrap();
}

fn main() {
    input! {
        n: usize,
        s: marker::Chars
    };
    let mut stdout = stdout().lock();
    solve(&mut stdout, n, s);
}

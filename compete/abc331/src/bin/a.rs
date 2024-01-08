use io::*;
use std::*;

macro_rules! mm {
    ($($x:ident),*) => {
        $(let mut $x = $x;)*
    };
}

fn solve(M: i64, D: i64, y: i64, m: i64, d: i64) {
    mm!(y, m, d);
    d += 1;
    if d > D {
        d = 1;
        m += 1;
    }
    if m > M {
        m = 1;
        y += 1;
    }
    println!("{} {} {}", y, m, d);
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut M: i64;
    M = scanner.next();
    let mut D: i64;
    D = scanner.next();
    let mut y: i64;
    y = scanner.next();
    let mut m: i64;
    m = scanner.next();
    let mut d: i64;
    d = scanner.next();
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(M, D, y, m, d))
        .unwrap()
        .join()
        .unwrap();
}

pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    }
}

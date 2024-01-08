use io::*;
use std::*;

fn solve(n: i64, m: i64, l: i64, a: Vec<i64>, b: Vec<i64>, c: Vec<i64>, d: Vec<i64>) {
    //
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut N: i64;
    N = scanner.next();
    let mut M: i64;
    M = scanner.next();
    let mut L: i64;
    L = scanner.next();
    let mut a: Vec<i64> = vec![0i64; (N) as usize];
    for i in 0..(N) as usize {
        a[i] = scanner.next();
    }
    let mut b: Vec<i64> = vec![0i64; (M) as usize];
    for i in 0..(M) as usize {
        b[i] = scanner.next();
    }
    let mut c: Vec<i64> = vec![0i64; (L) as usize];
    let mut d: Vec<i64> = vec![0i64; (L) as usize];
    for i in 0..(L) as usize {
        c[i] = scanner.next();
        d[i] = scanner.next();
    }
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(N, M, L, a, b, c, d)).unwrap().join().unwrap();
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

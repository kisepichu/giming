use io::*;
use std::*;

const MOD: i64 = 998244353;
fn solve(N: i64, M: i64, C: i64, B: Vec<i64>, A: Vec<Vec<i64>>) {
    //
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut N: i64;
    N = scanner.next();
    let mut M: i64;
    M = scanner.next();
    let mut C: i64;
    C = scanner.next();
    let mut B: Vec<i64> = vec![0i64; (M) as usize];
    for i in 0..(M) as usize {
        B[i] = scanner.next();
    }
    let mut A: Vec<Vec<i64>> = vec![vec![0i64; (M) as usize]; (N) as usize];
    for i in 0..(N) as usize {
        for j in 0..(M) as usize {
            A[i][j] = scanner.next();
        }
    }
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(N, M, C, B, A)).unwrap().join().unwrap();
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
        s.trim_right().to_owned()
    }
}
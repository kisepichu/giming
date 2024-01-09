use io::*;
use std::cmp::*;
use std::*;

fn solve(n: i64, k: i64, a: Vec<i64>) {
    let mut cur: usize = 0;
    let mut c = vec![];
    for i in 0..n {
        c.push(i + 1);
        if (cur < k as usize && a[cur] == i + 1) {
            cur += 1;
        } else {
            c.push(i + 1);
        }
    }
    let m = c.len();

    let mut d = vec![0i64; m - 1];
    for i in 0..m - 1 {
        d[i] = c[i + 1] - c[i];
    }
    let mut l = vec![0i64; m];
    for i in 0..m - 1 {
        l[i + 1] = l[i] + if i % 2 == 0 { d[i] } else { 0 };
    }

    if m % 2 == 0 {
        println!("{}", l.last().unwrap());
    } else {
        let mut r = vec![0i64; m];
        for i in (0..m - 1).rev() {
            r[i] = r[i + 1] + if i % 2 == 1 { d[i] } else { 0 };
        }
        let mut ans = 1e18 as i64;
        for i in 0..m {
            if (i % 2 == 0) {
                ans = min(ans, l[i] + r[i]);
            }
        }
        println!("{}", ans);
    }
}

fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut N: i64;
    N = scanner.next();
    let mut K: i64;
    K = scanner.next();
    let mut A: Vec<i64> = vec![0i64; (K) as usize];
    for i in 0..(K) as usize {
        A[i] = scanner.next();
    }
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(N, K, A)).unwrap().join().unwrap();
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

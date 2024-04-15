use proconio::{input, marker::Usize1 as usize1};
use std::io::Write;

fn solve(n: usize) {
    println!("Hello, {}!", n);
}

fn main() {
    input! {
        n: usize
    }
    solve(n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = 7;
        let expected = "Hello, 7!\n";
    }
}

use proconio::{input, fastout};
fn solve(M: i64, N: i64, A: Vec<i64>, B: Vec<i64>) {
  //
}
#[fastout]
fn main() {
  let M: i64;
  let N: i64;
  {
    input! { tmp: i64 };
    N = tmp;
  }
  {
    input! { tmp: i64 };
    M = tmp;
  }
  let A: Vec<i64> = vec![i64; N];
  let B: Vec<i64> = vec![i64; N];
  for i0 in 0..N {
    {
      input! { tmp: i64 };
      A[i0] = tmp;
    }
    {
      input! { tmp: i64 };
      B[i0] = tmp;
    }
  }
  solve(M, N, A, B);
}

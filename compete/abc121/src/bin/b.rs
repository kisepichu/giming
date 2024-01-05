use proconio::{input, fastout};
fn solve(C: i64, M: i64, N: i64, A: Vec<Vec<i64>>, B: Vec<i64>) {
  //
}
#[fastout]
fn main() {
  let C: i64;
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
  {
    input! { tmp: i64 };
    C = tmp;
  }
  let A: Vec<Vec<i64>> = vec![vec![i64; M]; N];
  let B: Vec<i64> = vec![i64; M];
  for i0 in 0..M {
    {
      input! { tmp: i64 };
      B[i0] = tmp;
    }
  }
  for i1 in 0..N {
    for i0 in 0..M {
      {
        input! { tmp: i64 };
        A[i1][i0] = tmp;
      }
    }
  }
  solve(C, M, N, A, B);
}

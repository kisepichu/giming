use proconio::{fastout, input};
fn solve(H: i64, W: i64, h: i64, w: i64) {
    println!("{}", (H - h) * (W - w));
}
#[fastout]
fn main() {
    let H: i64;
    let W: i64;
    let h: i64;
    let w: i64;
    {
        input! { tmp: i64 };
        H = tmp;
    }
    {
        input! { tmp: i64 };
        W = tmp;
    }
    {
        input! { tmp: i64 };
        h = tmp;
    }
    {
        input! { tmp: i64 };
        w = tmp;
    }
    solve(H, W, h, w);
}

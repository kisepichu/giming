use proconio::input;
fn solve<W: Write>(b: i64, g: i64) {
    println!("{}", if b > g { "Bat" } else { "Glove" });
}

fn main() {
    input! {
        b: i64,
        g: i64,
    };
    solve(b, g);
}

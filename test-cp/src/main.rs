use proconio::input;

fn main() {
    input! {
        b: i64,
        g: i64
    }
    println!("{}", if b < g { "Glove" } else { "Bat" });
}

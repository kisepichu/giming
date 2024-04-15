use proconio::{input, marker::Usize1 as usize1};

macro_rules! out {
    ($($arg:tt)*) => ({
        use std::io::Write;
        write!($crate::get_output(), $($arg)*).unwrap();
    });
}

struct MockStdout {
    buffer: String,
}

impl MockStdout {
    fn new() -> MockStdout {
        MockStdout {
            buffer: String::new(),
        }
    }

    fn get_output(&self) -> &str {
        &self.buffer
    }
}

impl std::io::Write for MockStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.push_str(std::str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn solve(n: usize) {
    for i in 0..n {
        out!("{}", if i % 3 == 2 { "x" } else { "o" });
    }
}
fn main() {
    input! {
        n: usize
    }
    {
        let mut stdout = std::io::stdout().lock();
        let get_output = || &mut stdout;
        solve(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        {
            let input = 7;
            let expected = "ooxooxo";
            let mut mock_stdout = MockStdout::new();
            {
                let _ = mock_stdout.get_output();
                solve(input);
            }
            assert_eq!(expected, mock_stdout.get_output());
        }
    }
}

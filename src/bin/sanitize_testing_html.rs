use regex::Regex;
use rstest::rstest;
use std::error::Error;
use std::io::{Read, Write};

#[derive(Debug)]
enum AppError {
    Regex(regex::Error),
    Io(std::io::Error),
    Path,
    #[allow(unused)]
    RegexNotFound,
    Unknown,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Regex(e) => write!(f, "Regex error: {}", e),
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Path => write!(f, "Path error"),
            AppError::RegexNotFound => write!(f, "Regex not found"),
            AppError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Regex(e) => Some(e),
            AppError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(e: regex::Error) -> Self {
        AppError::Regex(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

fn replace_tokens(s: String) -> Result<String, AppError> {
    let patterns = [r"[A-Za-z0-9+/]{13,}={1,2}", r"[A-Za-z0-9\-+_]{13,}"];
    let exclude_patterns = [r"[a-z\-]+", r#"[A-Z][a-z]*(\-[A-Z][a-z]*)*"#];

    let pattern = patterns.join("|");
    let exclude_pattern = exclude_patterns.join("|").to_string();

    let r = Regex::new(pattern.as_str())?;
    let re = Regex::new(exclude_pattern.as_str())?;

    let mut result = s.clone();
    for m in r.find_iter(s.as_str()) {
        let matched = m.as_str();
        let find = re.find(matched);
        if find.is_some() && matched.len() == find.ok_or(AppError::Unknown)?.as_str().len() {
            continue;
        }
        result = result.replace(matched, "SANITIZED");
    }
    Ok(result)
}

fn process_file(path: &str) -> Result<(), AppError> {
    println!("path: {:?}", path);
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let sanitized_path = path.replace(".html", ".sanitized.html");
    let mut file = std::fs::File::create(sanitized_path)?;
    file.write_all(replace_tokens(contents)?.as_bytes())?;
    Ok(())
}

fn process_dir(dir: &str) -> Result<(), AppError> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = path.to_str().ok_or(AppError::Path)?;
        if path.is_dir() {
            process_dir(path_str)?;
        } else if path_str.ends_with(".html") && !path_str.ends_with(".sanitized.html") {
            process_file(path_str)?;
        }
    }
    Ok(())
}

fn main() {
    let res = process_dir("crates/infrastructure/tests/responses");
    if let Err(e) = res {
        println!("{:?}", e);
        let mut source = e.source();
        while let Some(e) = source {
            println!("{:?}", e);
            source = e.source();
        }
    }
}

// #[test]
// fn test_replace_tokens() {
//     let testcases = vec![
//         ("yh87_G7c5PMrA4DrSpdY_4DrSpdYmsCz4msCz4", "SANITIZED"),
//         ("UWV0aTU5NjVEcVRuY0VjTGpxTjdqSlhVCg==", "SANITIZED"),
//         (
//             r#"div id="contest-table-permanent""#,
//             r#"div id="contest-table-permanent""#,
//         ),
//     ];

//     for (input, expected) in testcases {
//         assert_eq!(replace_tokens(input.to_string()), expected);
//     }
// }

#[rstest(
    input,
    expected,
    case("yh87_G7c5PMrA4DrSpdY_4DrSpdYmsCz4msCz4", "SANITIZED"),
    case("UWV0aTU5NjVEcVRuY0VjTGpxTjdqSlhVCg==", "SANITIZED"),
    case(
        r#"div id="contest-table-permanent""#,
        r#"div id="contest-table-permanent""#,
    )
)]
fn test_replace_tokens(input: &str, expected: &str) {
    assert_eq!(replace_tokens(input.to_string()).unwrap(), expected);
}

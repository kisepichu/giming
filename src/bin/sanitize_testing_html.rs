use regex::Regex;
use rstest::rstest;
use std::io::{Read, Write};

fn replace_tokens(s: String) -> String {
    let patterns = [r"[A-Za-z0-9+/]{13,}={1,2}", r"[A-Za-z0-9\-+_]{13,}"];
    let exclude_patterns = [r"[a-z\-]+", r#"[A-Z][a-z]*(\-[A-Z][a-z]*)*"#];

    let pattern = patterns.join("|").to_string();
    let exclude_pattern = exclude_patterns.join("|").to_string();

    let r = Regex::new(pattern.as_str()).unwrap();
    let re = Regex::new(exclude_pattern.as_str()).unwrap();

    let mut result = s.clone();
    for m in r.find_iter(s.as_str()) {
        let matched = m.as_str();
        if matched.len() == re.find(matched).unwrap().as_str().len() {
            continue;
        }
        result = result.replace(matched, "SANITIZED");
    }
    result
}

fn process_file(path: &str) {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sanitized_path = path.replace(".html", ".sanitized.html");
    let mut file = std::fs::File::create(sanitized_path).unwrap();
    file.write_all(replace_tokens(contents).as_bytes()).unwrap();
}

fn process_dir(dir: &str) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            process_dir(path.to_str().unwrap());
        } else if path.to_str().unwrap().ends_with(".html")
            && !path.to_str().unwrap().ends_with(".sanitized.html")
        {
            process_file(path.to_str().unwrap());
        }
    }
}

fn main() {
    process_dir("tests/responses");
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
    assert_eq!(replace_tokens(input.to_string()), expected);
}

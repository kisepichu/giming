pub fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("http") {
        contest_id_or_url
            .split('/')
            .next_back()
            .expect(
                "No panic because contest_id_or_url starts with http,
so the split must have at least one element",
            )
            .to_string()
    } else {
        contest_id_or_url
    }
}

const ATCODER_SUFFIX: [&str; 6] = ["abc", "arc", "agc", "typical", "tdpc", "edpc"];

pub fn oj_name_from_contest_id(contest_id: &str) -> Option<&str> {
    for suffix in ATCODER_SUFFIX.iter() {
        if contest_id.starts_with(suffix) {
            return Some("AtCoder");
        }
    }
    None
}

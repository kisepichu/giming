use chrono::{Date, DateTime, TimeZone, Utc};
use clap::Parser;
use std::fs;
use std::process::Output;
use std::{env, path::Path};
use std::{io::BufRead, process::Command};
use tera::Tera;

const F5_INTERVAL_MS: u64 = 100;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct InitArgs {
    /// Contest id or url
    contest: String,

    /// Solution files to overwrite when exists
    #[clap(short, long, default_value = "")]
    overwrite: String,
}

fn change_dir(path: &str) {
    env::set_current_dir(Path::new(path)).unwrap();
}

fn to_contest_id(contest_id_or_url: String) -> String {
    if contest_id_or_url.starts_with("https://atcoder.jp/contests/") {
        let split = contest_id_or_url.split("/").collect::<Vec<&str>>();
        let contest_id_index = if split.len() > 5 { 5 } else { split.len() - 1 };
        split[contest_id_index].to_string()
    } else {
        contest_id_or_url
    }
}

fn cargo_compete_new(contest_id: &String) -> Output {
    Command::new("cargo")
        .arg("compete")
        .arg("new")
        .arg(contest_id)
        .output()
        .expect("failed to execute process")
}

fn create_contest_dir_after_start(contest_id: &String) -> Output {
    let maybe_before = cargo_compete_new(&contest_id);
    return if maybe_before.status.success() {
        maybe_before
    } else {
        let stderr_last_line = maybe_before.stderr.lines().last().unwrap().unwrap();
        if !stderr_last_line.contains("will begin at") {
            // コンテスト開始前エラー以外のエラー
            panic!("cargo compete failed: {}", stderr_last_line)
        }

        // 開始時刻 1 秒前まで、 10 秒おき(秒数 1 の位が 0 の時)に現在時刻を表示
        let start_time_str = stderr_last_line
            .split("will begin at ")
            .collect::<Vec<&str>>()[1];
        let start_time = DateTime::parse_from_str(start_time_str, "%Y-%m-%d %H:%M:%S %z").unwrap();
        while start_time.timestamp() - Utc::now().timestamp() > 1 {
            if Utc::now().timestamp() % 10 == 0 {
                println!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        // 成功するまで F5
        let mut result = cargo_compete_new(&contest_id);
        while !result.status.success() {
            std::thread::sleep(std::time::Duration::from_millis(F5_INTERVAL_MS));
            result = cargo_compete_new(&contest_id);
        }
        result
    };
}

struct TestCase {
    input: String,
    output: String,
}

struct SolutionContext {
    contest_id: String,
    problem_id: String,
    testcases: Vec<TestCase>,
}

pub fn init(args: InitArgs) {
    println!("Init: {:?}", args);

    let contest_id = to_contest_id(args.contest);

    change_dir("compete");

    create_contest_dir_after_start(&contest_id);

    change_dir(&contest_id);

    let mut paths: Vec<_> = fs::read_dir("src/bin")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    paths.sort_by_key(|x| x.file_name());

    for file in paths {
        if file.file_type().unwrap().is_file() {
            let file_name = file.file_name();
            // .rs を消す
            let problem_id = file_name
                .to_str()
                .unwrap()
                .to_string()
                .split('.')
                .next()
                .unwrap()
                .to_string();
            println!("{}_{}", contest_id, problem_id);
        }
    }
}

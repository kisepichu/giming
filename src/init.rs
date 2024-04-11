use chrono::{Date, DateTime, TimeZone, Utc};
use clap::Parser;
use std::process::Output;
use std::{env, path::Path};
use std::{io::BufRead, process::Command};

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

// url の場合 contest_id に変換する
fn to_contest_id(s: String) -> String {
    if s.starts_with("https://atcoder.jp/contests/") {
        let split = s.split("/").collect::<Vec<&str>>();
        let contest_id_index = if split.len() > 5 { 5 } else { split.len() - 1 };
        split[contest_id_index].to_string()
    } else {
        s
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

fn create_contest_dir(contest_id: &String) {
    let maybe_before = cargo_compete_new(&contest_id);
    let result = if maybe_before.status.success() {
        maybe_before
    } else {
        let stderr_last_line = maybe_before.stderr.lines().last().unwrap().unwrap();
        if !stderr_last_line.contains("will begin at") {
            // コンテスト開始前エラー以外のエラー
            panic!("cargo compete failed: {}", stderr_last_line)
        }

        // 開始時刻まで待つ
        let start_time_str = stderr_last_line
            .split("will begin at ")
            .collect::<Vec<&str>>()[1];
        let start_time = DateTime::parse_from_str(start_time_str, "%Y-%m-%d %H:%M:%S %z").unwrap();
        // 開始時刻 1 秒前まで、 10 秒おき(秒数 1 の位が 0 の時)に現在時刻を表示
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

pub fn init(args: InitArgs) {
    println!("Init: {:?}", args);

    let contest_id = to_contest_id(args.contest);

    env::set_current_dir(Path::new("compete")).unwrap();

    // コンテストが開始するまで待ち、コンテスト用フォルダを生成する
    create_contest_dir(&contest_id);
}

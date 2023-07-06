use std::{fs, path::PathBuf, process::Command};

use tempfile::NamedTempFile;
use time::OffsetDateTime;

fn main() {
    let now = OffsetDateTime::now_utc();

    let metadata_path = NamedTempFile::new().unwrap().into_temp_path();
    let metadata = Command::new("cargo")
        .args(["metadata"])
        .output()
        .unwrap()
        .stdout;
    fs::write(&metadata_path, metadata).unwrap();

    let mut bench_path = PathBuf::from("benchmark_results");
    bench_path.push(&format!(
        "{}-{}-{}_{}-{}-{}",
        now.year(),
        now.month() as usize,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    ));

    let mut log_path = bench_path.clone();
    log_path.set_extension("log");
    let log = Command::new("cargo")
        .args(["bench"])
        .output()
        .unwrap()
        .stdout;
    fs::write(&log_path, log).unwrap();

    let mut json_path = bench_path.clone();
    json_path.set_extension("json");
    Command::new("cargo")
        .args(["run", "-p", "parser", "--", "--log"])
        .arg(&log_path)
        .arg("--meta")
        .arg(&metadata_path)
        .arg("--output")
        .arg(&json_path)
        .status()
        .unwrap();

    let mut config_path = PathBuf::from("tools");
    config_path.push("config.json");

    let mut template_path = PathBuf::from("tools");
    template_path.push("README.md.template");

    Command::new("cargo")
        .args(["run", "-p", "formatter", "--"])
        .arg(&json_path)
        .arg("--config")
        .arg(&config_path)
        .arg("--template")
        .arg(&template_path)
        .args([
            "--date",
            &format!(
                "{}-{}-{} {}:{}:{}",
                now.year(),
                now.month() as usize,
                now.day(),
                now.hour(),
                now.minute(),
                now.second(),
            ),
            "--output",
            "README.md",
        ])
        .status()
        .unwrap();

    metadata_path.close().unwrap();
}

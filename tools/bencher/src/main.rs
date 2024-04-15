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

    let mut rustc_info_path = PathBuf::from("runtime_info");
    rustc_info_path.push("rustc_info");
    let rustc_version = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .unwrap()
        .stdout;
    fs::write(&rustc_info_path, rustc_version).unwrap();

    #[cfg(target_os = "linux")]
    let cpu_info_path = {
        let mut cpu_info_path = PathBuf::from("runtime_info");
        cpu_info_path.push("cpu_info");
        let lscpu = Command::new("lscpu").output().unwrap().stdout;
        fs::write(&cpu_info_path, lscpu).unwrap();
        cpu_info_path
    };

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

    let mut config_path = PathBuf::from("tools");
    config_path.push("config.json");

    let mut json_path = bench_path.clone();
    json_path.set_extension("json");
    let parser = Command::new("cargo")
        .args(["run", "-p", "parser", "--"])
        .arg(&log_path)
        .arg("--config")
        .arg(&config_path)
        .arg("--meta")
        .arg(&metadata_path)
        .arg("--rustc-info")
        .arg(&rustc_info_path)
        .arg("--output")
        .arg(&json_path);
    #[cfg(target_os = "linux")]
    parser.arg("--lscpu").arg(&cpu_info_path);
    parser.status().unwrap();

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

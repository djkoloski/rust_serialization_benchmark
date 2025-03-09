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

    let rustc_info_path = NamedTempFile::new().unwrap().into_temp_path();
    let rustc_version = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .unwrap()
        .stdout;
    fs::write(&rustc_info_path, rustc_version).unwrap();

    #[cfg(target_os = "linux")]
    let cpu_info_path = {
        let cpu_info_path = NamedTempFile::new().unwrap().into_temp_path();
        let lscpu = Command::new("lscpu").output().unwrap().stdout;
        fs::write(&cpu_info_path, lscpu).unwrap();
        cpu_info_path
    };

    let mut bench_path = PathBuf::from("benchmark_results");
    bench_path.push(format!(
        "{yr}-{mon:02}-{day:02}_{hr:02}-{min:02}-{sec:02}",
        yr = now.year(),
        mon = now.month() as usize,
        day = now.day(),
        hr = now.hour(),
        min = now.minute(),
        sec = now.second(),
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
    let mut parser = Command::new("cargo");
    parser
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
    parser.arg("--cpu-info").arg(&cpu_info_path);
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
                "{yr}-{mon:02}-{day:02} {hr:02}:{min:02}:{sec:02}",
                yr = now.year(),
                mon = now.month() as usize,
                day = now.day(),
                hr = now.hour(),
                min = now.minute(),
                sec = now.second(),
            ),
            "--output",
            "README.md",
        ])
        .status()
        .unwrap();

    metadata_path.close().unwrap();
}

use eyre::{eyre as err, OptionExt};
use std::{
    fs::{self, OpenOptions},
    io::{BufReader, Read, Write},
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
};
use tempfile::NamedTempFile;
use time::OffsetDateTime;

struct Tee<R, W> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Read for Tee<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        self.writer.write_all(&buf[..n])?;
        Ok(n)
    }
}

trait Exit {
    fn exit_ok(self, name: &str) -> eyre::Result<()>;
}

impl Exit for ExitStatus {
    fn exit_ok(self, name: &str) -> eyre::Result<()> {
        self.success().ok_or_else(|| {
            err!(
                "{name} process exited with status {status:?}",
                status = self.code()
            )
        })
    }
}

fn main() -> eyre::Result<()> {
    let now = OffsetDateTime::now_utc();

    let metadata_path = NamedTempFile::new()?.into_temp_path();
    let metadata = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()?;
    metadata.status.exit_ok("cargo metadata")?;
    fs::write(&metadata_path, metadata.stdout)?;

    let rustc_info_path = NamedTempFile::new()?.into_temp_path();
    let rustc_version = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()?;
    rustc_version.status.exit_ok("rustc info")?;
    fs::write(&rustc_info_path, rustc_version.stdout)?;

    #[cfg(target_os = "linux")]
    let cpu_info_path = {
        let cpu_info_path = NamedTempFile::new()?.into_temp_path();
        let lscpu = Command::new("lscpu").output()?;
        lscpu.status.exit_ok("lscpu")?;
        fs::write(&cpu_info_path, lscpu.stdout)?;
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

    eprintln!("Running benchmarks:");
    eprintln!("-------------------");

    let mut log_path = bench_path.clone();
    log_path.set_extension("log");
    let mut benchmark_command = Command::new("cargo")
        .args(["bench"])
        .env("RUSTFLAGS", "-C target-cpu=native")
        .stdout(Stdio::piped())
        .spawn()?;
    let benchmark_stdout = std::io::BufReader::new(
        benchmark_command
            .stdout
            .as_mut()
            .ok_or_eyre("couldn't get benchmark stdout")?,
    );
    let mut stdout_tee = Tee {
        reader: BufReader::new(benchmark_stdout),
        writer: std::io::stdout(),
    };
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&log_path)?;
    std::io::copy(&mut stdout_tee, &mut log_file)?;
    benchmark_command.wait()?.exit_ok("benchmark")?;
    log_file.sync_all()?;
    drop(log_file);

    eprintln!("-------------------");
    eprintln!("Benchmarks complete");

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
    parser.status()?.exit_ok("parser")?;

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
        .status()?
        .exit_ok("formatter")?;

    Ok(())
}

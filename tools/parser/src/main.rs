use cargo_metadata::Metadata;
use clap::Parser;
use regex::Regex;
use std::{fs, path::PathBuf};

use schema::{Bench, Crate, Dataset, Meta, Results};

#[derive(Parser, Debug)]
#[command(name = "parser")]
#[command(about = "Parses benchmark logs from rust_serialization_benchmark into JSON", long_about = None)]
struct Args {
    #[arg(long)]
    log: PathBuf,
    #[arg(long)]
    meta: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn parse_time(s: &str) -> f64 {
    let (n, unit) = s.split_once(' ').unwrap();
    let factor = match unit {
        "ns" => 1.0,
        "µs" => 1_000.0,
        "ms" => 1_000_000.0,
        _ => panic!("unrecognized unit of time"),
    };
    n.parse::<f64>().unwrap() * factor
}

fn main() {
    let args = Args::parse();

    let log = fs::read_to_string(&args.log).unwrap();
    let metadata =
        serde_json::from_str::<Metadata>(&fs::read_to_string(args.meta).unwrap()).unwrap();

    let time_benches_re = Regex::new(
        r"(?m)^([a-z0-9_\-]+)\/([a-z_\-]+)\/([a-z\-]+)(?: \(([a-z \-+]*)\))?\s+time:   \[\d+\.\d+ [µnm]s (\d+\.\d+ [µnm]s)"
    ).unwrap();
    let size_benches_re =
        Regex::new(r"(?m)^([a-z0-9_\-]+)\/([a-z_\-]+)\/(size|zlib|zstd) (\d+)").unwrap();

    let mut results = Results::default();

    for capture in time_benches_re.captures_iter(&log) {
        let crate_name = &capture[2];

        let dataset = results
            .datasets
            .entry(capture[1].to_string())
            .or_insert(Dataset::default());
        let crate_ = dataset
            .crates
            .entry(crate_name.to_string())
            .or_insert(Crate::default());
        let bench = crate_
            .benches
            .entry(capture[3].to_string())
            .or_insert(Bench::nanos());
        let values = bench.unwrap_nanos();

        let value = parse_time(&capture[5]);
        if let Some(variant) = capture.get(4) {
            values.variants.insert(variant.as_str().to_string(), value);
        } else {
            values.primary = Some(value);
        }

        register_crate_version(&mut results.meta, crate_name, &metadata);
    }

    for capture in size_benches_re.captures_iter(&log) {
        let crate_name = &capture[2];

        let dataset = results
            .datasets
            .entry(capture[1].to_string())
            .or_insert(Dataset::default());
        let crate_ = dataset
            .crates
            .entry(crate_name.to_string())
            .or_insert(Crate::default());
        let bench = crate_
            .benches
            .entry(capture[3].to_string())
            .or_insert(Bench::bytes());
        let values = bench.unwrap_bytes();
        values.primary = Some(capture[4].parse().unwrap());

        register_crate_version(&mut results.meta, crate_name, &metadata);
    }

    fs::write(args.output, serde_json::to_string(&results).unwrap()).unwrap();
}

fn register_crate_version(meta: &mut Meta, crate_name: &str, metadata: &Metadata) {
    if !meta.crate_versions.contains_key(crate_name) {
        let version = metadata
            .packages
            .iter()
            .find(|pkg| pkg.name == crate_name)
            .unwrap()
            .version
            .clone();
        meta.crate_versions
            .insert(crate_name.to_string(), format!("{version}"));
    }
}

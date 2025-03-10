use cargo_metadata::Metadata;
use clap::Parser;
use regex::Regex;
use std::{fs, path::PathBuf};

use schema::{Bench, Config, Dataset, Feature, PackageId, Results};

#[derive(Parser, Debug)]
#[command(name = "parser")]
#[command(about = "Parses benchmark logs from rust_serialization_benchmark into JSON", long_about = None)]
struct Args {
    log: PathBuf,
    #[arg(long)]
    config: PathBuf,
    #[arg(long)]
    meta: PathBuf,
    #[arg(long)]
    rustc_info: PathBuf,
    #[arg(long)]
    cpu_info: Option<PathBuf>,
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
    let config = Config::read(&args.config);
    let metadata =
        serde_json::from_str::<Metadata>(&fs::read_to_string(args.meta).unwrap()).unwrap();
    let rustc_info = fs::read_to_string(&args.rustc_info).unwrap();
    let cpu_info = args
        .cpu_info
        .as_ref()
        .map(|path| fs::read_to_string(path).unwrap());

    let time_benches_re = Regex::new(
        r"(?m)^([a-z0-9_\-]+)\/([a-z0-9_\-]+)\/([a-z0-9_\-]+)(?: \(([a-z0-9_\-+ ]*)\))?\s+time:   \[\d+\.\d+ [µnm]s (\d+\.\d+ [µnm]s)"
    ).unwrap();
    let size_benches_re =
        Regex::new(r"(?m)^([a-z0-9_\-]+)\/([a-z0-9_\-]+)\/(size|zlib|zstd) (\d+)").unwrap();

    let mut results = Results {
        cpu_info,
        rustc_info,
        ..Default::default()
    };

    for capture in time_benches_re.captures_iter(&log) {
        let feature = &capture[2];
        results
            .features
            .entry(feature.to_string())
            .or_insert_with(|| find_package_id(feature, &config, &metadata));

        let dataset = results
            .datasets
            .entry(capture[1].to_string())
            .or_insert(Dataset::default());
        let package = dataset
            .features
            .entry(feature.to_string())
            .or_insert(Feature::default());
        let bench = package
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
    }

    for capture in size_benches_re.captures_iter(&log) {
        let feature = &capture[2];
        results
            .features
            .entry(feature.to_string())
            .or_insert_with(|| find_package_id(feature, &config, &metadata));

        let dataset = results
            .datasets
            .entry(capture[1].to_string())
            .or_insert(Dataset::default());
        let package = dataset
            .features
            .entry(feature.to_string())
            .or_insert(Feature::default());
        let bench = package
            .benches
            .entry(capture[3].to_string())
            .or_insert(Bench::bytes());
        let values = bench.unwrap_bytes();
        values.primary = Some(capture[4].parse().unwrap());
    }

    fs::write(args.output, serde_json::to_string(&results).unwrap()).unwrap();
}

fn find_package_id(feature: &str, config: &Config, metadata: &Metadata) -> PackageId {
    if let Some(package_id) = config.crate_matching.get(feature) {
        PackageId {
            name: package_id.name.clone(),
            version: find_package_version(
                &package_id.name,
                Some(
                    package_id
                        .version
                        .parse()
                        .expect("invalid version spec in config"),
                ),
                metadata,
            ),
        }
    } else {
        PackageId {
            name: feature.to_string(),
            version: find_package_version(feature, None, metadata),
        }
    }
}

fn find_package_version(
    name: &str,
    version_req: Option<semver::VersionReq>,
    metadata: &Metadata,
) -> String {
    metadata
        .packages
        .iter()
        .find(|pkg| {
            pkg.name == name
                && version_req
                    .as_ref()
                    .map_or(true, |req| req.matches(&pkg.version))
        })
        .unwrap()
        .version
        .to_string()
}

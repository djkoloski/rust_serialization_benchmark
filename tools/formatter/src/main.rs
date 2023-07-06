use std::{
    collections::HashMap,
    fmt::{self, Display, Write},
    fs,
    path::PathBuf,
};

use clap::Parser;

use schema::{Bench, Dataset, Meta, Results, Values};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "formatter")]
#[command(about = "Formats the README.md template using the parsed data from a benchmarking run")]
struct Args {
    input: PathBuf,
    #[arg(short, long)]
    config: PathBuf,
    #[arg(short, long)]
    template: PathBuf,
    #[arg(short, long)]
    date: String,
    #[arg(short, long)]
    output: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct Config {
    descriptions: HashMap<String, String>,
    do_not_edit: String,
}

fn main() {
    let args = Args::parse();

    let results =
        serde_json::from_str::<Results>(&fs::read_to_string(args.input).unwrap()).unwrap();
    let config = serde_json::from_str::<Config>(&fs::read_to_string(args.config).unwrap()).unwrap();
    let template = fs::read_to_string(args.template).unwrap();

    fs::write(
        args.output,
        format(&results, &config, &template, &args.date).unwrap(),
    )
    .unwrap();
}

struct Tables {
    header: String,
    data: String,
    comparison: String,
}

struct Nanos(f64);

impl Display for Nanos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, suffix) = if self.0 > 1_000_000.0 {
            (self.0 / 1_000_000.0, "ms")
        } else if self.0 > 1_000.0 {
            (self.0 / 1_000.0, "µs")
        } else {
            (self.0, "ns")
        };
        write!(f, "{value:.*} {suffix}", 4 - value.log10().floor() as usize)
    }
}

struct Bytes(u64);

impl Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Percent(f64, f64);

impl Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0 / self.1 * 100.0)
    }
}

fn format_values<T: Copy, U: Display>(
    values: &Values<T>,
    output: &mut String,
    display: impl Fn(T) -> U,
) -> fmt::Result {
    if let Some(value) = values.primary {
        write!(output, " {}", display(value))?;
    }

    for (name, value) in values.variants.iter() {
        write!(
            output,
            " <span title=\"{name}\">*{}\\**</span>",
            display(*value)
        )?;
    }
    write!(output, " |")?;
    Ok(())
}

fn write_crate_row(output: &mut String, crate_name: &str, meta: &Meta) -> fmt::Result {
    write!(
        output,
        "| [{crate_name} {}][{crate_name}] |",
        meta.crate_versions[crate_name]
    )
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn build_tables(
    dataset: &Dataset,
    columns: &[&str],
    meta: &Meta,
    placeholder: &str,
) -> Result<Tables, fmt::Error> {
    let mut header = "| Crate |".to_string();
    for column in columns {
        write!(&mut header, " {} |", capitalize(column))?;
    }
    write!(&mut header, "\n|---|")?;
    for _ in columns {
        write!(&mut header, "--:|")?;
    }

    let mut data = String::new();
    let mut comparison = String::new();

    let mins = columns
        .iter()
        .cloned()
        .map(|col| {
            dataset
                .crates
                .values()
                .filter_map(|crate_| crate_.benches.get(col))
                .map(|bench| match bench {
                    Bench::Nanos(values) => values.iter().cloned().reduce(f64::min).unwrap(),
                    Bench::Bytes(values) => values.iter().cloned().min().unwrap() as f64,
                })
                .reduce(f64::min)
                .unwrap()
        })
        .collect::<Vec<_>>();

    for (crate_name, crate_) in dataset.crates.iter() {
        if !columns.iter().all(|&c| crate_.benches.get(c).is_none()) {
            write_crate_row(&mut data, crate_name, meta)?;
            write_crate_row(&mut comparison, crate_name, meta)?;

            for (&column, &min) in columns.iter().zip(mins.iter()) {
                if let Some(bench) = crate_.benches.get(column) {
                    match bench {
                        Bench::Nanos(values) => {
                            format_values(values, &mut data, Nanos)?;
                            format_values(values, &mut comparison, |x| Percent(min, x))?;
                        }
                        Bench::Bytes(values) => {
                            format_values(values, &mut data, Bytes)?;
                            format_values(values, &mut comparison, |x| Percent(min, x as f64))?;
                        }
                    }
                } else {
                    write!(&mut data, " {placeholder} |")?;
                    write!(&mut comparison, " {placeholder} |")?;
                }
            }
            write!(&mut data, "\n")?;
            write!(&mut comparison, "\n")?;
        }
    }

    Ok(Tables {
        header,
        data,
        comparison,
    })
}

fn format(
    results: &Results,
    config: &Config,
    template: &str,
    date: &str,
) -> Result<String, fmt::Error> {
    const SERDE_COLS: &[&str] = &["serialize", "deserialize", "size", "zlib", "zstd"];
    const ZCD_COLS: &[&str] = &["access", "read", "update"];

    let mut tables = String::new();

    for (dataset_name, dataset) in results.datasets.iter() {
        let serde_tables = build_tables(dataset, SERDE_COLS, &results.meta, "†")?;
        let zcd_tables = build_tables(dataset, ZCD_COLS, &results.meta, "‡")?;

        write!(
            &mut tables,
            "\
            ## `{dataset_name}`\n\
            \n\
            {}\n\
            \n\
            ### Raw data\n\
            \n\
            For operations, time per iteration; for size, bytes. Lower is better.\n\
            \n\
            #### Serialize / deserialize speed and size\n\
            \n\
            {}\n\
            {}\n\
            #### Zero-copy deserialization speed\n\
            \n\
            {}\n\
            {}\n\
            ### Comparison\n\
            \n\
            Relative to best. Higher is better.\n\
            \n\
            #### Serialize / deserialize speed and size\n\
            \n\
            {}\n\
            {}\n\
            #### Zero-copy deserialization speed\n\
            \n\
            {}\n\
            {}\n\
            ",
            config
                .descriptions
                .get(dataset_name)
                .map(|desc| desc.as_str())
                .unwrap_or("Missing dataset description"),
            serde_tables.header,
            serde_tables.data,
            zcd_tables.header,
            zcd_tables.data,
            serde_tables.header,
            serde_tables.comparison,
            zcd_tables.header,
            zcd_tables.comparison,
        )?;
    }

    let mut links = String::new();
    for (crate_name, version) in results.meta.crate_versions.iter() {
        write!(
            &mut links,
            "[{crate_name}]: https://crates.io/crates/{crate_name}/{version}\n",
        )?;
    }

    Ok(template
        .replace("{dne}", &config.do_not_edit)
        .replace("{date}", date)
        .replace("{tables}", &tables)
        .replace("{links}", &links))
}

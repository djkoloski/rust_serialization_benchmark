use std::{
    borrow::Cow,
    collections::BTreeSet,
    fmt::{self, Display, Write},
    fs,
    path::PathBuf,
};

use clap::Parser;

use schema::{Bench, Config, Dataset, FeatureName, Features, Results, Values};

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

fn main() {
    let args = Args::parse();

    let config = Config::read(&args.config);

    let results =
        serde_json::from_str::<Results>(&fs::read_to_string(args.input).unwrap()).unwrap();
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
            " <span title=\"{name}\">*{val}\\**</span>",
            val = display(*value)
        )?;
    }
    write!(output, " |")?;
    Ok(())
}

fn write_crate_row(
    output: &mut String,
    feature: FeatureName<'_>,
    features: &Features,
) -> fmt::Result {
    let package_id = features.get(feature.name).unwrap();
    if let Some(encoding) = feature.common_encoding {
        write!(
            output,
            "| {encoding}:<br> [{pkg} {version}][{feature}] |",
            pkg = package_id.crate_name,
            version = package_id.version,
            feature = feature.name,
        )
    } else {
        write!(
            output,
            "| [{pkg} {version}][{feature}] |",
            pkg = package_id.crate_name,
            version = package_id.version,
            feature = feature.name,
        )
    }
}

pub fn capitalize(s: &str) -> String {
    s.split("_")
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_tables(
    features: &Features,
    dataset: &Dataset,
    config: &Config,
    columns: &[&str],
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
                .features
                .values()
                .filter_map(|feature| feature.benches.get(col))
                .map(|bench| match bench {
                    Bench::Nanos(values) => values.iter().cloned().reduce(f64::min).unwrap(),
                    Bench::Bytes(values) => values.iter().cloned().min().unwrap() as f64,
                })
                .reduce(f64::min)
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    for (feature, crate_) in dataset.grouped_features(config) {
        if columns.iter().any(|&c| crate_.benches.contains_key(c)) {
            write_crate_row(&mut data, feature, features)?;
            write_crate_row(&mut comparison, feature, features)?;

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
            writeln!(&mut data)?;
            writeln!(&mut comparison)?;
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
    const ALL_SER_DE_COLS: &[&str] = &[
        "serialize",
        "deserialize",
        "borrow",
        "size",
        "zlib",
        "zstd",
        "zstd_time",
    ];
    const ZCD_COLS: &[&str] = &["access", "read", "update"];

    let mut runtime_info = format!(
        "\
        ### `rustc` version\n\
        \n\
        ```\n\
        {rustc_info_trimmed}\n\
        ```",
        rustc_info_trimmed = results.rustc_info.trim_end(),
    );
    if let Some(cpu_info) = &results.cpu_info {
        write!(
            &mut runtime_info,
            "\n\
            \n\
            ### CPU info\n\
            \n\
            ```\n\
            {cpu_info_trimmed}\n\
            ```",
            cpu_info_trimmed = cpu_info.trim_end(),
        )?;
    }

    let mut tables = String::new();

    for (dataset_name, dataset) in results.datasets.iter() {
        let mut ser_de_cols = Cow::from(ALL_SER_DE_COLS);
        // Only keep the "borrow" column if the suite has a borrowable test data type
        if !config
            .suites
            .get(dataset_name)
            .is_some_and(|suite| suite.borrowable)
        {
            ser_de_cols.to_mut().retain(|&col| col != "borrow");
        }
        let serde_tables = build_tables(&results.features, dataset, config, &ser_de_cols, "†")?;
        let zcd_tables = build_tables(&results.features, dataset, config, ZCD_COLS, "‡")?;

        write!(
            &mut tables,
            "\
            ## `{dataset_name}`\n\
            \n\
            {dataset_description}\n\
            \n\
            ### Raw data\n\
            \n\
            For operations, time per iteration; for size, bytes. Lower is better.\n\
            \n\
            #### Serialize / deserialize speed and size\n\
            \n\
            {ser_de_header}\n\
            {ser_de_data}\n\
            #### Zero-copy deserialization speed\n\
            \n\
            {zcd_header}\n\
            {zcd_data}\n\
            ### Comparison\n\
            \n\
            Relative to best. Higher is better.\n\
            \n\
            #### Serialize / deserialize speed and size\n\
            \n\
            {ser_de_header}\n\
            {ser_de_comparison}\n\
            #### Zero-copy deserialization speed\n\
            \n\
            {zcd_header}\n\
            {zcd_comparison}\n\
            ",
            dataset_description = config
                .suites
                .get(dataset_name)
                .map(|suite| suite.description.as_str())
                .unwrap_or("Missing dataset description"),
            ser_de_header = serde_tables.header,
            ser_de_data = serde_tables.data,
            ser_de_comparison = serde_tables.comparison,
            zcd_header = zcd_tables.header,
            zcd_data = zcd_tables.data,
            zcd_comparison = zcd_tables.comparison,
        )?;
    }

    let mut links = String::new();
    let features = results
        .datasets
        .values()
        .flat_map(|dataset| dataset.features.keys())
        .collect::<BTreeSet<_>>();
    for &feature in features.iter() {
        writeln!(
            &mut links,
            "[{feature}]: {}",
            results.features.get(feature).unwrap().crates_io_url(),
        )?;
    }

    Ok(template
        .replace("{dne}", &config.do_not_edit_message)
        .replace("{date}", date)
        .replace("{runtime_info}", &runtime_info)
        .replace("{tables}", &tables)
        .replace("{links}", &links))
}

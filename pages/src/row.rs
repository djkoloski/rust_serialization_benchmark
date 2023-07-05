use crate::compression::{Compression, CompressionMap};
use std::str::FromStr;

#[derive(Debug)]
pub struct Row {
    pub crate_: String,
    pub version: String,
    pub serialize: f32,
    pub deserialize: Option<f32>,
    pub sizes: CompressionMap<u64>,
}

type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cols = s.split('|');
        let mut col = || cols.next().ok_or("missing '|''");
        col()?;

        let (crate_, version) = col()?
            .split_once('[')
            .ok_or("missing '['")?
            .1
            .split_once(']')
            .ok_or("missing ']'")?
            .0
            .split_once(' ')
            .ok_or("invalid crate + version")?;
        let crate_ = crate_.to_owned();
        let version = version.to_owned();

        let serialize = parse_seconds(col()?)?.ok_or("no serialize")?;
        let deserialize = parse_seconds(col()?)?;

        let mut sizes = CompressionMap::default();
        sizes.insert(Compression::None, parse_bytes(col()?)?);
        sizes.insert(Compression::Zlib, parse_bytes(col()?)?);
        sizes.insert(Compression::Zstd, parse_bytes(col()?)?);

        Ok(Row {
            crate_,
            version,
            serialize,
            deserialize,
            sizes,
        })
    }
}

fn parse_seconds(s: &str) -> Result<Option<f32>> {
    let s = s.trim();
    let s = if let Some((_, s)) = s.split_once(">*") {
        s.split_once('\\').ok_or("missing '\\'")?.0
    } else {
        s
    };

    let Some((num, unit)) = s.split_once(' ') else {
        return (s == "†").then_some(None).ok_or("invalid nanoseconds");
    };

    let num = f64::from_str(num).map_err(|_| "invalid f64")?;
    Ok(Some(
        (num / match unit {
            "ns" => 1_000_000_000.0,
            "µs" => 1_000_000.0,
            "ms" => 1_000.0,
            _ => return Err("unknown unit"),
        }) as f32,
    ))
}

fn parse_bytes(s: &str) -> Result<u64> {
    u64::from_str(s.trim()).map_err(|_| "invalid u64")
}

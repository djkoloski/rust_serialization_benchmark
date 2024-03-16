use crate::compression::{Compression, CompressionMap};
use schema::{Bench, Feature};

#[derive(Debug)]
pub struct Row {
    pub feature: String,
    pub serialize: f32,
    pub deserialize: Option<f32>,
    pub compression: CompressionMap<CompressionEntry>,
}

#[derive(Debug)]
pub struct CompressionEntry {
    pub size: u64,
    pub compress: Option<f64>,
}

type Error = &'static str;
type Result<T> = std::result::Result<T, Error>;

fn unwrap_bytes(bench: &Bench) -> Result<u64> {
    match bench {
        Bench::Bytes(v) => Ok(v.primary.ok_or("missing bytes")?),
        _ => Err("not bytes"),
    }
}

fn unwrap_seconds(bench: &Bench) -> Result<Option<f64>> {
    match bench {
        Bench::Nanos(v) => Ok(v.iter().next().map(|&ns| ns / 1_000_000_000.0)),
        _ => Err("not nanos"),
    }
}

impl TryFrom<(&String, &Feature)> for Row {
    type Error = Error;

    fn try_from((feature, Feature { benches }): (&String, &Feature)) -> Result<Self> {
        let col = |key: &'static str| -> Result<&Bench> { benches.get(key).ok_or(key) };

        let serialize = unwrap_seconds(col("serialize")?)?.ok_or("no serialize primary")? as f32;
        let deserialize = col("deserialize")
            .ok()
            .and_then(|v| unwrap_seconds(v).ok().flatten())
            .map(|v| v as f32);

        let mut compression = CompressionMap::default();
        compression.insert(
            Compression::None,
            CompressionEntry {
                size: unwrap_bytes(col("size")?)?,
                compress: None,
            },
        );
        compression.insert(
            Compression::Zlib,
            CompressionEntry {
                size: unwrap_bytes(col("zlib")?)?,
                compress: None,
            },
        );
        compression.insert(
            Compression::Zstd,
            CompressionEntry {
                size: unwrap_bytes(col("zstd")?)?,
                compress: unwrap_seconds(col("zstd_time")?)?,
            },
        );

        Ok(Self {
            feature: feature.clone(),
            serialize,
            deserialize,
            compression,
        })
    }
}

use crate::compression::{Compression, CompressionMap};
use schema::{Bench, Crate};

#[derive(Debug)]
pub struct Row {
    pub crate_: String,
    pub serialize: f32,
    pub deserialize: Option<f32>,
    pub sizes: CompressionMap<u64>,
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

impl TryFrom<(&String, &Crate)> for Row {
    type Error = Error;

    fn try_from((crate_, Crate { benches }): (&String, &Crate)) -> Result<Self> {
        let col = |key: &'static str| -> Result<&Bench> { benches.get(key).ok_or(key) };

        let serialize = unwrap_seconds(col("serialize")?)?.ok_or("no serialize primary")? as f32;
        let deserialize = col("deserialize")
            .ok()
            .and_then(|v| unwrap_seconds(v).ok().flatten())
            .map(|v| v as f32);

        let mut sizes = CompressionMap::default();
        sizes.insert(Compression::None, unwrap_bytes(col("size")?)?);
        sizes.insert(Compression::Zlib, unwrap_bytes(col("zlib")?)?);
        sizes.insert(Compression::Zstd, unwrap_bytes(col("zstd")?)?);

        Ok(Self {
            crate_: crate_.clone(),
            serialize,
            deserialize,
            sizes,
        })
    }
}

use crate::compression::{Compression, CompressionSet};
use crate::mode::Mode;
use crate::row::Row;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Bottleneck {
    Bandwidth,
    Cpu,
}

impl Display for Bottleneck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bandwidth => "bandwidth",
                Self::Cpu => "CPU",
            }
        )
    }
}

pub struct CalcRow {
    pub compression: Compression,
    pub feature: String,
    pub limit: Bottleneck,
    pub messages_per_second: f32,
    pub relative: f32,
}

pub fn calc(
    rows: Vec<Row>,
    messages_per_benchmark: u32,
    bandwidth: u64,
    cpus: f32,
    compression_set: &CompressionSet,
    mode: Mode,
) -> Vec<CalcRow> {
    let bandwidth_per_second = bandwidth as f32 * (1.0 / (30.4 * 24.0 * 60.0 * 60.0));

    let mut rows: Vec<_> = rows
        .into_iter()
        .filter_map(|r| {
            r.deserialize
                .or_else(|| (mode == Mode::Serialize).then_some(0.0))
                .map(|d| (r, d))
        })
        .flat_map(|(r, deserialize)| {
            let Row {
                feature,
                serialize,
                compression,
                ..
            } = r;
            let uncompressed_size = compression.get(Compression::None).unwrap().size;

            compression
                .into_iter()
                .filter(|(c, _)| compression_set.contains(*c))
                .map(move |(compression, entry)| {
                    let compressed_size = entry.size;
                    let compress_time = entry.compress.map(|v| v as f32);

                    // TODO this assumes that inbound bandwidth is equivalent to outbound bandwidth which isn't the case for many VPS.
                    let limit_size = bandwidth_per_second
                        / (compressed_size * if mode == Mode::RoundTrip { 2 } else { 1 }) as f32;

                    // Use measured compress_time if it exists, or estimate it with Compression::serialize_seconds.
                    let serialize_seconds = serialize
                        + compress_time
                            .unwrap_or_else(|| compression.serialize_seconds(uncompressed_size));
                    let deserialize_seconds =
                        deserialize + compression.deserialize_seconds(uncompressed_size);
                    let limit_speed = cpus
                        / match mode {
                            Mode::Serialize => serialize_seconds,
                            Mode::Deserialize => deserialize_seconds,
                            Mode::RoundTrip => serialize_seconds + deserialize_seconds,
                        };

                    let (benchmarks_per_second, limit) = if limit_size < limit_speed {
                        (limit_size, Bottleneck::Bandwidth)
                    } else {
                        (limit_speed, Bottleneck::Cpu)
                    };
                    CalcRow {
                        compression,
                        feature: feature.clone(),
                        limit,
                        messages_per_second: benchmarks_per_second * messages_per_benchmark as f32,
                        relative: 0.0,
                    }
                })
        })
        .collect();

    rows.sort_by(|a, b| {
        b.messages_per_second
            .partial_cmp(&a.messages_per_second)
            .unwrap()
    });

    let max = rows
        .iter()
        .map(|r| r.messages_per_second)
        .fold(0.0, f32::max);
    for row in &mut rows {
        row.relative = row.messages_per_second / max;
    }

    // Dedup crates.
    let mut seen = HashSet::new();
    rows.retain(|r| seen.insert(r.feature.clone()));

    rows
}

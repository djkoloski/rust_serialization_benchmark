use enum_iterator::IntoEnumIterator;
use fixed_map::{Key, Map, Set};
use std::fmt::{Display, Formatter};

pub type CompressionMap<T> = Map<Compression, T>;
pub type CompressionSet = Set<Compression>;

#[derive(Copy, Clone, Debug, IntoEnumIterator, Key)]
pub enum Compression {
    None,
    Zlib,
    Zstd,
}

impl Compression {
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::into_enum_iter()
    }

    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(self) -> bool {
        !self.is_none()
    }

    pub fn seconds(self, bytes: u64) -> f32 {
        // TODO real benchmarks (since speed is different on different data and cpus).
        const SCALE: f32 = 0.5;
        const ZLIB_MBPS: f32 = 31.375 * SCALE;
        const ZSTD_MBPS: f32 = 202.985 * SCALE;

        bytes as f32
            * match self {
                Self::None => 0.0,
                Self::Zlib => 1.0 / (ZLIB_MBPS * 1_000_000.0),
                Self::Zstd => 1.0 / (ZSTD_MBPS * 1_000_000.0),
            }
    }
}

impl Display for Compression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "",
                Self::Zlib => "zlib",
                Self::Zstd => "zstd",
            }
        )
    }
}

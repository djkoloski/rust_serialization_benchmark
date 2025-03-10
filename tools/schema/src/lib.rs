use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
};

#[derive(Clone, Deserialize, Serialize)]
pub struct PackageId {
    pub name: String,
    pub version: String,
}

impl PackageId {
    pub fn crates_io_url(&self) -> String {
        format!("https://crates.io/crates/{}/{}", self.name, self.version)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Suite {
    pub description: String,
    #[serde(default)]
    pub borrowable: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Benchmark suite information
    pub suites: HashMap<String, Suite>,
    /// Do-not-edit message inserted into the readme.md file warning contributors to modify the
    /// template instead
    pub do_not_edit: String,
    /// Information for distinguishing the appropriate crate for benchmarks whose name doesn't match
    /// the name of the crate, perhaps because multiple versions of that crate are being benchmarked
    pub crate_matching: HashMap<String, PackageId>,
    /// Information indicating which common encodings are implemented by the given libraries being
    /// benchmarked so that they can be compared side-by-side more directly; the keys are feature
    /// names and the values are names of common encodings
    pub equivalent_encodings: HashMap<String, String>,
}

impl Config {
    pub fn read(path: &Path) -> Self {
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Results {
    pub cpu_info: Option<String>,
    pub rustc_info: String,
    pub datasets: BTreeMap<String, Dataset>,
    pub features: Features,
}

pub type Features = BTreeMap<String, PackageId>;

#[derive(Default, Deserialize, Serialize)]
pub struct Dataset {
    pub features: BTreeMap<String, Feature>,
}

/// Represents the name of a benchmarked feature in the output, annotated with the name of the
/// common encoding it implements if we know it.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FeatureName<'a> {
    pub name: &'a str,
    pub common_encoding: Option<&'a str>,
}

impl Ord for FeatureName<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Order by:
        // 1. the common encoding (if we know it), falling back to the feature name
        // 2. features that have a known common encoding come first
        // 3. finally, within the same common encoding order by the feature name
        let this = (
            self.common_encoding.unwrap_or(self.name),
            self.common_encoding.is_none(),
            self.name,
        );
        let that = (
            other.common_encoding.unwrap_or(other.name),
            other.common_encoding.is_none(),
            other.name,
        );
        this.cmp(&that)
    }
}

impl PartialOrd<Self> for FeatureName<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Dataset {
    /// Groups features together with the given config, annotating them with the common encoding
    /// names from the config and returning a map that orders them according to that logic.
    pub fn grouped_features<'a>(&'a self, config: &'a Config) -> BTreeMap<FeatureName<'a>, &'a Feature> {
        self.features
            .iter()
            .map(|(name, feature)| {
                (
                    FeatureName {
                        name: name.as_str(),
                        common_encoding: config.equivalent_encodings.get(name).map(String::as_str),
                    },
                    feature,
                )
            })
            .collect()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Feature {
    pub benches: HashMap<String, Bench>,
}

#[derive(Deserialize, Serialize)]
pub enum Bench {
    Nanos(Values<f64>),
    Bytes(Values<u64>),
}

impl Bench {
    pub fn nanos() -> Bench {
        Bench::Nanos(Values::default())
    }

    pub fn bytes() -> Bench {
        Bench::Bytes(Values::default())
    }

    pub fn unwrap_nanos(&mut self) -> &mut Values<f64> {
        match self {
            Bench::Nanos(b) => b,
            _ => panic!("expected nanos bench"),
        }
    }

    pub fn unwrap_bytes(&mut self) -> &mut Values<u64> {
        match self {
            Bench::Bytes(b) => b,
            _ => panic!("expected nanos bench"),
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Values<T> {
    pub primary: Option<T>,
    pub variants: BTreeMap<String, T>,
}

impl<T> Values<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.variants.values().chain(self.primary.as_ref())
    }
}

use serde::{Deserialize, Serialize};
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
pub struct Config {
    pub descriptions: HashMap<String, String>,
    pub do_not_edit: String,
    pub features: HashMap<String, PackageId>,
}

impl Config {
    pub fn read(path: &Path) -> Self {
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Results {
    pub datasets: BTreeMap<String, Dataset>,
    pub features: Features,
}

pub type Features = BTreeMap<String, PackageId>;

#[derive(Default, Deserialize, Serialize)]
pub struct Dataset {
    pub features: BTreeMap<String, Feature>,
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

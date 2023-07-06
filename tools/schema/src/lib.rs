use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Default, Deserialize, Serialize)]
pub struct Results {
    pub datasets: BTreeMap<String, Dataset>,
    pub meta: Meta,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Meta {
    pub crate_versions: BTreeMap<String, String>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Dataset {
    pub crates: BTreeMap<String, Crate>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Crate {
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

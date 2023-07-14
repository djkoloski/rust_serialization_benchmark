use enum_iterator::IntoEnumIterator;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, IntoEnumIterator)]
pub enum Mode {
    Serialize,
    Deserialize,
    RoundTrip,
}

impl Mode {
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::into_enum_iter()
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Serialize => "serialized and sent",
            Self::Deserialize => "received and deserialized",
            Self::RoundTrip => "received, deserialized, serialized and sent",
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Serialize => "serialize",
                Self::Deserialize => "deserialize",
                Self::RoundTrip => "round trip",
            }
        )
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "serialize" => Self::Serialize,
            "deserialize" => Self::Deserialize,
            "round trip" => Self::RoundTrip,
            _ => return Err(()),
        })
    }
}

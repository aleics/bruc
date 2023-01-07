use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

pub trait DataSource {
  fn get(&self, key: &str) -> Option<&DataItem>;
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataItem {
  Bool(bool),
  Number(f32),
}

impl Display for DataItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DataItem::Bool(value) => write!(f, "{value}"),
      DataItem::Number(value) => write!(f, "{value}"),
    }
  }
}

impl Hash for DataItem {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      DataItem::Bool(value) => state.write_i8(*value as i8),
      DataItem::Number(value) => state.write(&value.to_be_bytes()),
    };
    state.finish();
  }
}

impl PartialEq for DataItem {
  fn eq(&self, other: &Self) -> bool {
    match self {
      DataItem::Bool(value) => {
        if let DataItem::Bool(other_value) = other {
          value == other_value
        } else {
          false
        }
      }
      DataItem::Number(value) => {
        if let DataItem::Number(other_value) = other {
          value == other_value
        } else {
          false
        }
      }
    }
  }
}

impl Eq for DataItem {}

impl From<bool> for DataItem {
  fn from(value: bool) -> Self {
    DataItem::Bool(value)
  }
}

impl From<f32> for DataItem {
  fn from(value: f32) -> Self {
    DataItem::Number(value)
  }
}

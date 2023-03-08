use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

pub trait DataSource {
  fn get(&self, key: &str) -> Option<&DataItem>;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataItem {
  Bool(bool),
  Number(f32),
  Text(String),
}

impl DataItem {
  pub fn get_number(&self) -> Option<&f32> {
    if let DataItem::Number(value) = &self {
      Some(value)
    } else {
      None
    }
  }

  pub fn get_bool(&self) -> Option<&bool> {
    if let DataItem::Bool(value) = &self {
      Some(value)
    } else {
      None
    }
  }

  pub fn get_text(&self) -> Option<&String> {
    if let DataItem::Text(value) = &self {
      Some(value)
    } else {
      None
    }
  }
}

impl Display for DataItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      DataItem::Bool(value) => write!(f, "{value}"),
      DataItem::Number(value) => write!(f, "{value}"),
      DataItem::Text(value) => write!(f, "{value}"),
    }
  }
}

impl Hash for DataItem {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    match self {
      DataItem::Bool(value) => hasher.write_i8(i8::from(*value)),
      DataItem::Number(value) => hasher.write(&value.to_be_bytes()),
      DataItem::Text(value) => value.hash(hasher),
    };
    hasher.finish();
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
      DataItem::Text(value) => {
        if let DataItem::Text(other_value) = other {
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

impl From<&str> for DataItem {
  fn from(value: &str) -> Self {
    DataItem::Text(value.to_string())
  }
}

impl From<String> for DataItem {
  fn from(value: String) -> Self {
    DataItem::Text(value)
  }
}

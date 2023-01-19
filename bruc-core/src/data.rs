use std::{collections::HashMap, fmt::Display};

use crate::transform::pipe::Pipe;
use bruc_expression::data::{DataItem, DataSource};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DataEntry {
  pub(crate) name: String,
  pub(crate) values: Vec<DataValue>,
  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) transform: Vec<Pipe>,
}

impl DataEntry {
  pub fn new(name: &str, values: Vec<DataValue>, transform: Vec<Pipe>) -> Self {
    DataEntry {
      name: name.to_string(),
      values,
      transform,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DataValue {
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub(crate) instance: HashMap<String, DataItem>,
}

impl DataValue {
  pub fn new() -> DataValue {
    DataValue {
      instance: HashMap::new(),
    }
  }

  pub fn with_instance(instance: HashMap<String, DataItem>) -> DataValue {
    DataValue { instance }
  }

  pub fn from_pairs(pairs: Vec<(&str, DataItem)>) -> DataValue {
    let mut vars = DataValue::new();
    for (key, var) in pairs {
      vars.insert(key, var);
    }
    vars
  }

  pub fn insert(&mut self, key: &str, value: DataItem) {
    self.instance.insert(key.to_string(), value);
  }

  pub fn pairs(&self) -> Vec<(&str, DataItem)> {
    self
      .instance
      .iter()
      .map(|(key, value)| (key.as_str(), *value))
      .collect()
  }
}

impl DataSource for DataValue {
  fn get(&self, key: &str) -> Option<&DataItem> {
    self.instance.get(key)
  }
}

impl Default for DataValue {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for DataValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut keys = self.instance.keys().cloned().collect::<Vec<String>>();
    keys.sort();

    let result = keys
      .iter()
      .map(|key| format!("\"{}\": {}", key, self.instance.get(key).unwrap()))
      .collect::<Vec<String>>();

    if result.is_empty() {
      write!(f, "{{}}")
    } else {
      write!(f, "{{ {} }}", result.join(", "))
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::data::{DataEntry, DataValue};

  #[test]
  fn deserialize_data_value() {
    let data_value: DataValue = serde_json::from_str(r#"{ "a": 2.0 }"#).unwrap();
    assert_eq!(data_value, DataValue::from_pairs(vec![("a", 2.0.into())]));
  }

  #[test]
  fn deserializes_data() {
    let data: Vec<DataEntry> = serde_json::from_str(
      r#"[{
        "name": "my_data",
        "values": [{"a": 3.0, "b": true }]
      }]"#,
    )
    .unwrap();

    assert_eq!(
      data,
      vec![DataEntry::new(
        "my_data",
        vec![DataValue::from_pairs(vec![
          ("a", 3.0.into()),
          ("b", true.into())
        ])],
        Vec::new()
      )]
    );
  }
}

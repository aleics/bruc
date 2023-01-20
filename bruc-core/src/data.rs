use std::{collections::HashMap, fmt::Display};

use bruc_expression::data::{DataItem, DataSource};

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

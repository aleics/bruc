use std::collections::HashMap;

use crate::data::{DataItem, DataSource};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Variables {
  #[cfg_attr(feature = "serde", serde(flatten))]
  instance: HashMap<String, DataItem>,
}

impl Variables {
  pub fn new() -> Variables {
    Variables {
      instance: HashMap::new(),
    }
  }

  pub fn with_instance(instance: HashMap<String, DataItem>) -> Variables {
    Variables { instance }
  }

  pub fn from_pairs(pairs: Vec<(&str, DataItem)>) -> Variables {
    let mut vars = Variables::new();
    for (key, var) in pairs {
      vars.insert(key, var);
    }
    vars
  }

  pub fn find(&self, key: &str) -> Option<&DataItem> {
    self.instance.get(key)
  }

  pub fn has(&self, key: &str) -> bool {
    self.instance.contains_key(key)
  }

  pub fn insert(&mut self, key: &str, value: DataItem) {
    self.instance.insert(key.to_string(), value);
  }
}

impl Default for Variables {
  fn default() -> Self {
    Variables::new()
  }
}

impl DataSource for Variables {
  fn get(&self, key: &str) -> Option<&DataItem> {
    self.find(key)
  }
}

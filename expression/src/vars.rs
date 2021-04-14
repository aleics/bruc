use std::collections::HashMap;

use crate::data::{DataItem, DataSource};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Variables<'a> {
  #[cfg_attr(feature = "serde", serde(flatten))]
  #[cfg_attr(feature = "serde", serde(borrow))]
  instance: HashMap<&'a str, DataItem>,
}

impl<'a> Variables<'a> {
  pub fn new() -> Variables<'a> {
    Variables {
      instance: HashMap::new(),
    }
  }

  pub fn with_instance(instance: HashMap<&'a str, DataItem>) -> Variables<'a> {
    Variables { instance }
  }

  pub fn from_pairs(pairs: Vec<(&'a str, DataItem)>) -> Variables<'a> {
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

  pub fn insert(&mut self, key: &'a str, value: DataItem) {
    self.instance.insert(key, value);
  }
}

impl<'a> Default for Variables<'a> {
  fn default() -> Self {
    Variables::new()
  }
}

impl<'a> DataSource for Variables<'a> {
  fn get(&self, key: &str) -> Option<&DataItem> {
    self.find(key)
  }
}

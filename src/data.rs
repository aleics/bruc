use ebooler::data::{DataItem, DataSource};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct DataValue<'a> {
  instance: HashMap<&'a str, DataItem>,
}

impl<'a> DataValue<'a> {
  pub fn new() -> DataValue<'a> {
    DataValue {
      instance: HashMap::new(),
    }
  }

  pub fn with_instance(instance: HashMap<&'a str, DataItem>) -> DataValue<'a> {
    DataValue { instance }
  }

  pub fn from_pairs(pairs: Vec<(&'a str, DataItem)>) -> DataValue<'a> {
    let mut vars = DataValue::new();
    for (key, var) in pairs {
      vars.insert(key, var);
    }
    vars
  }

  pub fn find(&self, key: &str) -> Option<&DataItem> {
    self.instance.get(key)
  }

  pub fn insert(&mut self, key: &'a str, value: DataItem) {
    self.instance.insert(key, value);
  }
}

impl<'a> DataSource for DataValue<'a> {
  fn get(&self, key: &str) -> Option<&DataItem> {
    self.instance.get(key)
  }
}

impl<'a> Default for DataValue<'a> {
  fn default() -> Self {
    Self::new()
  }
}

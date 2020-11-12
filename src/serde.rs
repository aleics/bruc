use crate::data::DataValue;
use ebooler::data::DataItem;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

impl<'de, 'a> Deserialize<'de> for DataValue<'a>
where
  'de: 'a,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let instance: HashMap<&str, DataItem> = HashMap::deserialize(deserializer)?;
    Ok(DataValue::with_instance(instance))
  }
}

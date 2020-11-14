use std::collections::HashMap;
use std::fmt;

use ebooler::data::DataItem;
use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::{de, Deserialize, Deserializer};

use crate::transform::data::DataValue;
use crate::transform::filter::FilterPipe;
use crate::transform::group::{GroupPipe, Operation};
use crate::transform::map::MapPipe;
use crate::transform::pipe::Pipe;
use crate::transform::Source;

impl<'de: 'a, 'a> Deserialize<'de> for Source<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct SourceVisitor;

    impl<'a> Visitor<'a> for SourceVisitor {
      type Value = Source<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Source")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut data = None;
        let mut pipes = None;

        while let Some(key) = map.next_key()? {
          match key {
            "data" => {
              if data.is_some() {
                return Err(de::Error::duplicate_field("data"));
              }
              data = map.next_value().ok();
            }
            "pipes" => {
              if pipes.is_some() {
                return Err(de::Error::duplicate_field("pipes"));
              }
              pipes = map.next_value().ok();
            }
            _ => {}
          }
        }

        let data = data.ok_or(de::Error::missing_field("data"))?;
        let pipes = pipes.ok_or(de::Error::missing_field("pipes"))?;

        Ok(Source::new(data, pipes))
      }
    }

    deserializer.deserialize_any(SourceVisitor)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for Pipe<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct PipeVisitor;

    impl<'a> Visitor<'a> for PipeVisitor {
      type Value = Pipe<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct Pipe")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        if let Some(key) = map.next_key()? {
          match key {
            "filter" => Ok(Pipe::Filter(map.next_value()?)),
            "map" => Ok(Pipe::Map(map.next_value()?)),
            "group" => Ok(Pipe::Group(map.next_value()?)),
            _ => Err(de::Error::unknown_variant(key, &["filter", "map", "group"])),
          }
        } else {
          Err(de::Error::custom("empty object"))
        }
      }
    }

    deserializer.deserialize_any(PipeVisitor)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for MapPipe<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct MapPipeVisitor;

    impl<'a> Visitor<'a> for MapPipeVisitor {
      type Value = MapPipe<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct MapPipe")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut predicate = None;
        let mut output = None;

        while let Some((key, value)) = map.next_entry()? {
          match key {
            "fn" => {
              if predicate.is_some() {
                return Err(de::Error::duplicate_field("fn"));
              }
              predicate = value;
            }
            "output" => {
              if output.is_some() {
                return Err(de::Error::duplicate_field("output"));
              }
              output = value;
            }
            _ => {}
          }
        }

        let predicate = predicate.ok_or(de::Error::missing_field("fn"))?;
        let output = output.ok_or(de::Error::missing_field("output"))?;

        MapPipe::new(predicate, output).map_err(|err| de::Error::custom(err.to_string()))
      }
    }

    deserializer.deserialize_any(MapPipeVisitor)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for FilterPipe<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct FilterPipeVisitor;

    impl<'a> Visitor<'a> for FilterPipeVisitor {
      type Value = FilterPipe<'a>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid predicate (string)")
      }

      #[inline]
      fn visit_borrowed_str<E: serde::de::Error>(self, value: &'a str) -> Result<Self::Value, E> {
        FilterPipe::new(value).map_err(|error| serde::de::Error::custom(error.to_string()))
      }
    }

    deserializer.deserialize_any(FilterPipeVisitor)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for GroupPipe<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct GroupPipeVisitor;

    impl<'a> Visitor<'a> for GroupPipeVisitor {
      type Value = GroupPipe<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct GroupPipe")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut by = None;
        let mut op = None;
        let mut output = None;

        while let Some((key, value)) = map.next_entry()? {
          match key {
            "by" => {
              if by.is_some() {
                return Err(de::Error::duplicate_field("by"));
              }
              by = value;
            }
            "op" => {
              if op.is_some() {
                return Err(de::Error::duplicate_field("op"));
              }
              op = value;
            }
            "output" => {
              if output.is_some() {
                return Err(de::Error::duplicate_field("output"));
              }
              output = value;
            }
            _ => {}
          }
        }

        let by = by.ok_or(de::Error::missing_field("by"))?;
        let op = op.ok_or(de::Error::missing_field("op"))?;
        let output = output.ok_or(de::Error::missing_field("output"))?;

        let op = Operation::from_string(op).ok_or(de::Error::unknown_variant(op, &["count"]))?;

        Ok(GroupPipe::new(by, op, output))
      }
    }

    deserializer.deserialize_any(GroupPipeVisitor)
  }
}

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

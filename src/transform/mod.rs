use crate::data::DataValue;
use crate::transform::pipe::{chain, Pipe, PipeStream};

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Source<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  data: Vec<DataValue<'a>>,
  #[cfg_attr(feature = "serde", serde(borrow))]
  pipes: Vec<Pipe<'a>>,
}

impl<'a> Source<'a> {
  pub fn new(data: Vec<DataValue<'a>>, pipes: Vec<Pipe<'a>>) -> Source<'a> {
    Source { data, pipes }
  }

  pub fn data(&self) -> &Vec<DataValue<'a>> {
    &self.data
  }

  pub fn pipes(&self) -> &Vec<Pipe<'a>> {
    &self.pipes
  }
}

pub fn run<'a>(source: &'a Source<'a>) -> PipeStream<'a> {
  chain(source.data(), source.pipes())
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::data::DataValue;
  use crate::transform::filter::FilterPipe;
  use crate::transform::group::{GroupPipe, Operation};
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;
  use crate::transform::Source;

  #[test]
  fn deserializes_source() {
    let source_json = r#"{
      "data": [
        { "a": 2.0 },
        { "a": 4.0 },
        { "a": 6.0 }
      ],
      "pipes": [
        { "filter": "a > 2" },
        { "map": { "fn": "a + 2", "output": "b" } },
        { "group": { "by": "b", "op": "count", "output": "count" } }
       ]
    }"#;
    let source: Source = serde_json::from_str(source_json).unwrap();

    assert_eq!(
      source.data(),
      &vec![
        DataValue::from_pairs(vec![("a", 2.0.into())]),
        DataValue::from_pairs(vec![("a", 4.0.into())]),
        DataValue::from_pairs(vec![("a", 6.0.into())])
      ]
    );
    assert_eq!(
      source.pipes(),
      &vec![
        Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
        Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
        Pipe::Group(GroupPipe::new("b", Operation::Count, "count"))
      ]
    );
  }
}

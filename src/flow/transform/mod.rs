use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::filter::FilterNode;
use crate::flow::transform::group::GroupNode;
use crate::flow::transform::map::MapNode;
use crate::transform::pipe::Pipe;
use futures::Stream;

pub mod filter;
pub mod group;
pub mod map;

#[inline]
pub fn chain<'a, S>(source: S, pipes: &'a [Pipe<'a>]) -> DataStream<'a>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  pipes
    .iter()
    .fold(Box::new(source), |acc, pipe| chain_transform(acc, pipe))
}

#[inline]
fn chain_transform<'a, S>(source: S, pipe: &'a Pipe<'a>) -> DataStream<'a>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  match pipe {
    Pipe::Filter(pipe) => Box::new(FilterNode::new(source, pipe)),
    Pipe::Map(pipe) => Box::new(MapNode::new(source, pipe)),
    Pipe::Group(pipe) => Box::new(GroupNode::new(source, pipe)),
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::chain;
  use crate::transform::filter::FilterPipe;
  use crate::transform::group::{GroupOperator, GroupPipe};
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;

  #[test]
  fn chain_empty() {
    let pipes: [Pipe; 0] = [];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 1.0.into())]),
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 3.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())])
        ]
      )
    });
  }

  #[test]
  fn chain_maps() {
    let pipes = [
      Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
      Pipe::Map(MapPipe::new("a + 4", "c").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![
            ("a", 1.0.into()),
            ("b", 3.0.into()),
            ("c", 5.0.into())
          ]),
          DataValue::from_pairs(vec![
            ("a", 2.0.into()),
            ("b", 4.0.into()),
            ("c", 6.0.into())
          ]),
          DataValue::from_pairs(vec![
            ("a", 3.0.into()),
            ("b", 5.0.into()),
            ("c", 7.0.into())
          ]),
          DataValue::from_pairs(vec![
            ("a", 4.0.into()),
            ("b", 6.0.into()),
            ("c", 8.0.into())
          ])
        ]
      )
    });
  }

  #[test]
  fn chain_filters() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Filter(FilterPipe::new("a < 4").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 3.0.into())])]);
    });
  }

  #[test]
  fn chain_groups() {
    let pipes = [
      Pipe::Group(GroupPipe::new("a", GroupOperator::Count, "a_count")),
      Pipe::Group(GroupPipe::new(
        "a_count",
        GroupOperator::Count,
        "count_a_count",
      )),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let values = Chunks::new(node).collect::<Vec<_>>().await;

      assert_eq!(values.len(), 2);
      assert!(values.contains(&DataValue::from_pairs(vec![
        ("a_count", 2.0.into()),
        ("count_a_count", 1.0.into())
      ])));
      assert!(values.contains(&DataValue::from_pairs(vec![
        ("a_count", 1.0.into()),
        ("count_a_count", 2.0.into())
      ])));
    });
  }

  #[test]
  fn chain_filter_map() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Map(MapPipe::new("a * 2", "b").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let values = Chunks::new(node).collect::<Vec<_>>().await;
      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 8.0.into())])
        ]
      );
    });
  }

  #[test]
  fn chain_filter_group() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Group(GroupPipe::new("a", GroupOperator::Count, "a_count")),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = chain(source.link(), &pipes);

    source.send(data);
    futures::executor::block_on(async {
      let result = Chunks::new(node).collect::<Vec<_>>().await;
      assert_eq!(result.len(), 2);
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a", 3.0.into()),
        ("a_count", 1.0.into())
      ])));
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a", 4.0.into()),
        ("a_count", 1.0.into())
      ])));
    });
  }
}

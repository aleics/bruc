use crate::data::DataValue;
use crate::flow::data::{Chunks, DataStream};
use crate::transform::group::{GroupOperator, GroupPipe};
use bruc_expreter::data::{DataItem, DataSource};
use futures::stream::LocalBoxStream;
use futures::task::{Context, Poll};
use futures::{FutureExt, Stream, StreamExt};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::pin::Pin;

pub struct GroupNode<S> {
  source: S,
}

impl<'a, S> GroupNode<S> {
  pub fn new(source: S) -> GroupNode<S> {
    GroupNode { source }
  }
}

impl<'a, S> GroupNode<S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  #[inline]
  pub fn chain(source: S, pipe: &'a GroupPipe<'a>) -> DataStream<'a> {
    let group_source = match pipe.op() {
      GroupOperator::Count => CountNode::chain(source, pipe),
    };

    Box::new(GroupNode::new(group_source))
  }
}

impl<S> Unpin for GroupNode<S> {}

impl<'a, S> Stream for GroupNode<S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  type Item = Option<DataValue<'a>>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

struct CountNode<'a> {
  source: RepsNode<'a>,
  by: &'a str,
  output: &'a str,
}

impl<'a> CountNode<'a> {
  pub fn new<S>(data: S, by: &'a str, output: &'a str) -> CountNode<'a>
  where
    S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
  {
    CountNode {
      source: RepsNode::new(data, by),
      by,
      output,
    }
  }

  #[inline]
  fn chain<S>(source: S, pipe: &'a GroupPipe<'a>) -> DataStream<'a>
  where
    S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
  {
    Box::new(CountNode::new(source, pipe.by(), pipe.output()))
  }
}

impl<'a> Unpin for CountNode<'a> {}

impl<'a> Stream for CountNode<'a> {
  type Item = Option<DataValue<'a>>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx).map(|value| {
      value.map(|value| {
        value.map(|(var, count)| {
          DataValue::from_pairs(vec![
            (self.by, var),
            (self.output, DataItem::Number(count as f32)),
          ])
        })
      })
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

struct RepsNode<'a> {
  source: LocalBoxStream<'a, Option<(DataItem, usize)>>,
}

impl<'a> RepsNode<'a> {
  pub fn new<S>(data: S, by: &'a str) -> RepsNode<'a>
  where
    S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
  {
    RepsNode {
      source: RepsNode::reps(data, by),
    }
  }

  #[inline]
  fn reps<S>(data: S, by: &'a str) -> LocalBoxStream<'a, Option<(DataItem, usize)>>
  where
    S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
  {
    Chunks::new(data)
      .fold(
        HashMap::<DataItem, usize>::new(),
        move |mut acc, value| async move {
          let target = value.get(by).copied();
          if let Some(target) = target {
            match acc.get_mut(&target) {
              Some(count) => count.add_assign(1),
              None => {
                acc.insert(target, 1);
              }
            }
          }
          acc
        },
      )
      .map(|data| {
        let mut result: Vec<Option<(DataItem, usize)>> = Vec::new();
        for item in data {
          result.push(Some(item));
        }
        result.push(None);

        futures::stream::iter(result)
      })
      .flatten_stream()
      .boxed_local()
  }
}

impl<'a> Unpin for RepsNode<'a> {}

impl<'a> Stream for RepsNode<'a> {
  type Item = Option<(DataItem, usize)>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::group::GroupNode;
  use crate::transform::group::{GroupOperator, GroupPipe};

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", GroupOperator::Count, "count");
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];
    let source = Source::new();
    let mut node = GroupNode::chain(Box::new(source.link()), &group);

    source.send(data);
    futures::executor::block_on(async {
      // let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        vec![node.next().await],
        vec![Some(Some(DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 2.0.into())
        ])))]
      )
    });
  }

  #[test]
  fn finds_no_repetition() {
    let group = GroupPipe::new("a", GroupOperator::Count, "count");
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("b", 3.0.into())]),
    ];
    let source = Source::new();
    let node = GroupNode::chain(Box::new(source.link()), &group);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 1.0.into())
        ])]
      )
    });
  }
}

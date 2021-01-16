use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::transform::group::{GroupOperator, GroupPipe};
use bruc_expreter::data::{DataItem, DataSource};
use futures::task::{Context, Poll};
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::pin::Pin;

pub enum GroupNode<'a, S> {
  Count(CountNode<'a, S>),
}

impl<'a, S> GroupNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  pub fn new(source: S, pipe: &'a GroupPipe<'a>) -> GroupNode<'a, S> {
    match pipe.op() {
      GroupOperator::Count => GroupNode::Count(CountNode::new(source, pipe.by(), pipe.output())),
    }
  }

  #[inline]
  pub fn chain(source: S, pipe: &'a GroupPipe<'a>) -> DataStream<'a> {
    Box::new(GroupNode::new(source, pipe))
  }
}

impl<'a, S> Unpin for GroupNode<'a, S> {}

impl<'a, S> Stream for GroupNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  type Item = Option<DataValue<'a>>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    match self.get_mut() {
      GroupNode::Count(node) => node.poll_next_unpin(cx),
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      GroupNode::Count(node) => node.size_hint(),
    }
  }
}

impl<'a, S> Clone for GroupNode<'a, S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    match self {
      GroupNode::Count(node) => GroupNode::Count(node.clone()),
    }
  }
}

pub struct CountNode<'a, S> {
  source: RepsNode<'a, S>,
  by: &'a str,
  output: &'a str,
}

impl<'a, S> CountNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  pub fn new(data: S, by: &'a str, output: &'a str) -> CountNode<'a, S> {
    CountNode {
      source: RepsNode::new(data, by),
      by,
      output,
    }
  }
}

impl<'a, S> Unpin for CountNode<'a, S> {}

impl<'a, S> Stream for CountNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
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

impl<'a, S> Clone for CountNode<'a, S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    CountNode {
      source: self.source.clone(),
      by: &self.by,
      output: &self.output,
    }
  }
}

struct RepsNode<'a, S> {
  source: S,
  tail: Option<HashMap<DataItem, usize>>,
  by: &'a str,
}

impl<'a, S> RepsNode<'a, S> {
  pub fn new(source: S, by: &'a str) -> RepsNode<'a, S> {
    RepsNode {
      source,
      tail: None,
      by,
    }
  }

  #[inline]
  pub fn count_value(&self, acc: &mut HashMap<DataItem, usize>, value: DataValue) {
    if let Some(target) = value.get(self.by) {
      match acc.get_mut(&target) {
        Some(count) => count.add_assign(1),
        None => {
          acc.insert(*target, 1);
        }
      }
    }
  }
}

impl<'a, S> Stream for RepsNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin + 'a,
{
  type Item = Option<(DataItem, usize)>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if self.tail.is_none() {
      self.tail = {
        let mut acc = HashMap::<DataItem, usize>::new();

        loop {
          match self.source.poll_next_unpin(cx) {
            Poll::Pending => break None,
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Ready(Some(Some(value))) => self.count_value(&mut acc, value),
            Poll::Ready(Some(None)) => break Some(acc),
          }
        }
      };
    }

    if let Some(tail) = self.tail.as_mut() {
      if tail.is_empty() {
        self.tail = None;
        Poll::Ready(Some(None))
      } else {
        let entry = tail
          .keys()
          .next()
          .cloned()
          .map(|key| tail.remove_entry(&key));

        Poll::Ready(entry)
      }
    } else {
      Poll::Pending
    }
  }
}

impl<'a, S> Clone for RepsNode<'a, S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    RepsNode {
      source: self.source.clone(),
      tail: None,
      by: &self.by,
    }
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
    let node = GroupNode::new(source.link(), &group);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 2.0.into())
        ])]
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
    let node = GroupNode::new(source.link(), &group);

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

use crate::data::DataValue;
use crate::transform::group::{GroupOperator, GroupPipe};
use expression::data::{DataItem, DataSource};
use futures::task::{Context, Poll};
use futures::{Stream, StreamExt};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::pin::Pin;

pub enum GroupNode<S> {
  Count(CountNode<S>),
}

impl<S> GroupNode<S> {
  pub fn new(source: S, pipe: GroupPipe) -> GroupNode<S> {
    match pipe.op {
      GroupOperator::Count => GroupNode::Count(CountNode::new(source, &pipe.by, &pipe.output)),
    }
  }
}

impl<S> Unpin for GroupNode<S> {}

impl<S> Stream for GroupNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<DataValue>;

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

impl<S> Clone for GroupNode<S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    match self {
      GroupNode::Count(node) => GroupNode::Count(node.clone()),
    }
  }
}

pub struct CountNode<S> {
  source: S,
  tail: Option<HashMap<DataItem, usize>>,
  by: String,
  output: String,
}

impl<S> CountNode<S> {
  pub fn new(source: S, by: &str, output: &str) -> CountNode<S> {
    CountNode {
      source,
      tail: None,
      by: by.to_string(),
      output: output.to_string(),
    }
  }

  #[inline]
  fn count_value(&self, acc: &mut HashMap<DataItem, usize>, value: DataValue) {
    if let Some(target) = value.get(&self.by) {
      match acc.get_mut(&target) {
        Some(count) => count.add_assign(1),
        None => {
          acc.insert(*target, 1);
        }
      }
    }
  }

  #[inline]
  fn next_tail_value(&mut self) -> Option<Option<DataValue>> {
    if let Some(tail) = self.tail.as_mut() {
      if tail.is_empty() {
        self.tail = None;
        Some(None)
      } else {
        tail
          .keys()
          .next()
          .cloned()
          .map(|key| tail.remove_entry(&key))
          .map(|entry| {
            entry.map(|(var, count)| {
              DataValue::from_pairs(vec![
                (&self.by, var),
                (&self.output, DataItem::Number(count as f32)),
              ])
            })
          })
      }
    } else {
      None
    }
  }
}

impl<S> Stream for CountNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<DataValue>;

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

    let value = self.next_tail_value();
    if value.is_some() {
      Poll::Ready(value)
    } else {
      Poll::Pending
    }
  }
}

impl<'a, S> Clone for CountNode<S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    CountNode {
      source: self.source.clone(),
      tail: None,
      by: self.by.clone(),
      output: self.output.clone(),
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
    let node = GroupNode::new(source.link(), group);

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
    let node = GroupNode::new(source.link(), group);

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

  #[test]
  fn clones() {
    let group = GroupPipe::new("a", GroupOperator::Count, "count");
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];

    let source = Source::new();

    let first = GroupNode::new(source.link(), group);
    let second = first.clone();

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(first).collect().await;
      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 2.0.into())
        ])]
      );

      let values: Vec<_> = Chunks::new(second).collect().await;
      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 2.0.into())
        ])]
      );
    });
  }
}

use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::TransformNode;
use crate::transform::group::{GroupOperator, GroupPipe};
use bruc_expreter::data::DataItem;
use futures::stream::LocalBoxStream;
use futures::task::{Context, Poll};
use futures::{FutureExt, Stream, StreamExt};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::pin::Pin;

pub struct GroupNode<'a> {
  source: DataStream<'a>,
}

impl<'a> GroupNode<'a> {
  pub fn new(source: DataStream<'a>) -> GroupNode<'a> {
    GroupNode { source }
  }

  #[inline]
  pub fn chain(source: TransformNode<'a>, pipe: &'a GroupPipe<'a>) -> TransformNode<'a> {
    let group_source = match pipe.op() {
      GroupOperator::Count => CountNode::chain(source, pipe),
    };

    let node = GroupNode::new(Box::new(group_source));
    TransformNode::new(Box::new(node))
  }
}

impl<'a> Unpin for GroupNode<'a> {}

impl<'a> Stream for GroupNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        break source;
      }
    })
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
  pub fn new(data: TransformNode<'a>, by: &'a str, output: &'a str) -> CountNode<'a> {
    CountNode {
      source: RepsNode::new(data, by),
      by,
      output,
    }
  }

  #[inline]
  fn chain(source: TransformNode<'a>, pipe: &'a GroupPipe<'a>) -> TransformNode<'a> {
    let node = CountNode::new(source, pipe.by(), pipe.output());
    TransformNode::new(Box::new(node))
  }
}

impl<'a> Unpin for CountNode<'a> {}

impl<'a> Stream for CountNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        let result = source.map(|(var, count)| {
          DataValue::from_pairs(vec![
            (self.by, var),
            (self.output, DataItem::Number(count as f32)),
          ])
        });

        break result;
      }
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

struct RepsNode<'a> {
  source: LocalBoxStream<'a, (DataItem, usize)>,
}

impl<'a> RepsNode<'a> {
  pub fn new(data: TransformNode<'a>, by: &'a str) -> RepsNode<'a> {
    RepsNode {
      source: RepsNode::reps(data, by),
    }
  }

  fn reps(data: TransformNode<'a>, by: &'a str) -> LocalBoxStream<'a, (DataItem, usize)> {
    data
      .fold(
        HashMap::<DataItem, usize>::new(),
        move |mut acc, item| async move {
          if let Some(target) = item.find(by) {
            match acc.get_mut(target) {
              Some(count) => count.add_assign(1),
              None => {
                acc.insert(*target, 1);
              }
            }
          }

          acc
        },
      )
      .map(|data| futures::stream::iter(data.into_iter()))
      .flatten_stream()
      .boxed_local()
  }
}

impl<'a> Unpin for RepsNode<'a> {}

impl<'a> Stream for RepsNode<'a> {
  type Item = (DataItem, usize);

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        break source;
      }
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::source_finite;
  use crate::flow::transform::group::GroupNode;
  use crate::flow::transform::TransformNode;
  use crate::transform::group::{GroupOperator, GroupPipe};

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", GroupOperator::Count, "count");
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];
    let source = source_finite(data);
    let node = GroupNode::chain(TransformNode::new(source), &group);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 2.0.into())
        ]),]
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
    let source = source_finite(data);
    let node = GroupNode::chain(TransformNode::new(source), &group);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("count", 1.0.into())
        ]),]
      )
    });
  }
}

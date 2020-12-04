use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::TransformStream;
use crate::transform::group::{GroupOperator, GroupPipe};
use bruc_expreter::data::DataItem;
use futures::stream::LocalBoxStream;
use futures::task::{Context, Poll};
use futures::{FutureExt, Stream, StreamExt};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::pin::Pin;

pub struct GroupStream<'a> {
  source: DataStream<'a>,
}

impl<'a> GroupStream<'a> {
  pub fn new(source: DataStream<'a>) -> GroupStream<'a> {
    GroupStream { source }
  }

  #[inline]
  pub fn chain(source: TransformStream<'a>, pipe: &'a GroupPipe<'a>) -> TransformStream<'a> {
    let group_source = match pipe.op() {
      GroupOperator::Count => CountStream::chain(source, pipe),
    };

    let stream = GroupStream::new(Box::new(group_source));
    TransformStream::new(Box::new(stream))
  }
}

impl<'a> Unpin for GroupStream<'a> {}

impl<'a> Stream for GroupStream<'a> {
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

struct CountStream<'a> {
  source: RepsStream<'a>,
  by: &'a str,
  output: &'a str,
}

impl<'a> CountStream<'a> {
  pub fn new(data: TransformStream<'a>, by: &'a str, output: &'a str) -> CountStream<'a> {
    CountStream {
      source: RepsStream::new(data, by),
      by,
      output,
    }
  }

  #[inline]
  fn chain(source: TransformStream<'a>, pipe: &'a GroupPipe<'a>) -> TransformStream<'a> {
    let stream = CountStream::new(source, pipe.by(), pipe.output());
    TransformStream::new(Box::new(stream))
  }
}

impl<'a> Unpin for CountStream<'a> {}

impl<'a> Stream for CountStream<'a> {
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

struct RepsStream<'a> {
  source: LocalBoxStream<'a, (DataItem, usize)>,
}

impl<'a> RepsStream<'a> {
  pub fn new(data: TransformStream<'a>, by: &'a str) -> RepsStream<'a> {
    RepsStream {
      source: RepsStream::reps(data, by),
    }
  }

  fn reps(data: TransformStream<'a>, by: &'a str) -> LocalBoxStream<'a, (DataItem, usize)> {
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

impl<'a> Unpin for RepsStream<'a> {}

impl<'a> Stream for RepsStream<'a> {
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
  use crate::flow::transform::group::GroupStream;
  use crate::flow::transform::TransformStream;
  use crate::transform::group::{GroupOperator, GroupPipe};

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", GroupOperator::Count, "count");
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];
    let source = source_finite(data);
    let stream = GroupStream::chain(TransformStream::new(source), &group);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

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
    let stream = GroupStream::chain(TransformStream::new(source), &group);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

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

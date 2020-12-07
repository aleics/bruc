use crate::flow::data::DataStream;
use crate::scale::linear::LinearScale;
use crate::scale::Scaler;
use bruc_expreter::data::DataSource;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct LinearNode<'a> {
  source: DataStream<'a>,
  scale: LinearScale<'a>,
  field: &'a str,
}

impl<'a> LinearNode<'a> {
  pub fn new(source: DataStream<'a>, scale: LinearScale<'a>, field: &'a str) -> LinearNode<'a> {
    LinearNode {
      source,
      scale,
      field,
    }
  }
}

impl<'a> Unpin for LinearNode<'a> {}

impl<'a> Stream for LinearNode<'a> {
  type Item = f32;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        match source {
          Some(item) => {
            let result = item
              .get(self.field)
              .and_then(|value| self.scale.scale(value));

            if result.is_some() {
              break result;
            }
          }
          None => break None,
        }
      }
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::source_finite;
  use crate::flow::scale::linear::LinearNode;
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use futures::StreamExt;

  #[test]
  fn applies() {
    let source = source_finite(vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ]);

    let scale = LinearScale::new(
      "horizontal",
      Domain::Literal(0.0, 10.0),
      Range::Literal(0.0, 1.0),
    );

    let node = LinearNode::new(source, scale, "x");

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;
      assert_eq!(values, vec![0.0, 0.5, 1.0, 1.0]);
    })
  }

  #[test]
  fn ignores_boolean() {
    let source = source_finite(vec![
      DataValue::from_pairs(vec![("x", true.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", false.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 2.0.into()), ("y", 1.0.into())]),
    ]);

    let scale = LinearScale::new(
      "horizontal",
      Domain::Literal(0.0, 10.0),
      Range::Literal(0.0, 1.0),
    );

    let node = LinearNode::new(source, scale, "x");

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;
      assert_eq!(values, vec![0.2]);
    })
  }
}

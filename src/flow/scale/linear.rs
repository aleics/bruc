use crate::data::DataValue;
use crate::scale::linear::LinearScale;
use crate::scale::Scaler;
use bruc_expreter::data::DataSource;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct LinearNode<'a, S> {
  source: S,
  scale: LinearScale<'a>,
  field: &'a str,
}

impl<'a, S> LinearNode<'a, S> {
  pub fn new(source: S, scale: LinearScale<'a>, field: &'a str) -> LinearNode<'a, S> {
    LinearNode {
      source,
      scale,
      field,
    }
  }
}

impl<'a, S> Unpin for LinearNode<'a, S> {}

impl<'a, S> Stream for LinearNode<'a, S>
where
  S: Stream<Item = Option<DataValue<'a>>> + Unpin,
{
  type Item = Option<f32>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        match source {
          Some(item) => {
            if let Some(value) = item {
              let result = value
                .get(self.field)
                .and_then(|value| self.scale.scale(value));

              if result.is_some() {
                break Some(result);
              }
            } else {
              break Some(None);
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
  use crate::flow::data::{Chunks, Source};
  use crate::flow::scale::linear::LinearNode;
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use futures::StreamExt;

  #[test]
  fn applies() {
    let data = vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ];

    let scale = LinearScale::new(
      "horizontal",
      Domain::Literal(0.0, 10.0),
      Range::Literal(0.0, 1.0),
    );

    let source = Source::new();
    let node = LinearNode::new(source.link(), scale, "x");

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;
      assert_eq!(values, vec![0.0, 0.5, 1.0, 1.0]);
    })
  }

  #[test]
  fn ignores_boolean() {
    let data = vec![
      DataValue::from_pairs(vec![("x", true.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", false.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 2.0.into()), ("y", 1.0.into())]),
    ];

    let scale = LinearScale::new(
      "horizontal",
      Domain::Literal(0.0, 10.0),
      Range::Literal(0.0, 1.0),
    );

    let source = Source::new();
    let node = LinearNode::new(Box::new(source.link()), scale, "x");

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;
      assert_eq!(values, vec![0.2]);
    })
  }
}

use crate::data::DataValue;
use crate::spec::scale::linear::LinearScale;
use crate::spec::scale::Scaler;
use bruc_expression::data::DataSource;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct LinearNode<S> {
  source: S,
  scale: LinearScale,
  field: String,
}

impl<S> LinearNode<S> {
  pub fn new(source: S, scale: LinearScale, field: &str) -> LinearNode<S> {
    LinearNode {
      source,
      scale,
      field: field.to_string(),
    }
  }
}

impl<S> Unpin for LinearNode<S> {}

impl<S> Stream for LinearNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<f32>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        match source {
          Some(item) => {
            if let Some(value) = item {
              let result = value
                .get(&self.field)
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
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use futures::StreamExt;

  #[test]
  fn applies() {
    let data = vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ];

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

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

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let source = Source::new();
    let node = LinearNode::new(source.link(), scale, "x");

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;
      assert_eq!(values, vec![0.2]);
    })
  }
}

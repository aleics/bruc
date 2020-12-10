use crate::data::DataValue;
use futures::channel::mpsc;
use futures::task::{Context, Poll};
use futures::{SinkExt, Stream};
use std::pin::Pin;

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub fn source_finite(data: Vec<DataValue>) -> DataStream {
  Box::new(futures::stream::iter(data.into_iter()))
}

pub fn source<'a>() -> (Source<'a>, DataStream<'a>) {
  let (sender, receiver) = mpsc::unbounded();
  (Source::new(sender), SourceNode::chain(receiver))
}

pub struct SourceNode<'a> {
  receiver: mpsc::UnboundedReceiver<DataValue<'a>>,
}

impl<'a> SourceNode<'a> {
  fn chain(receiver: mpsc::UnboundedReceiver<DataValue<'a>>) -> DataStream<'a> {
    Box::new(SourceNode { receiver })
  }
}

impl<'a> Unpin for SourceNode<'a> {}

impl<'a> Stream for SourceNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(item) = Pin::new(&mut self.receiver).poll_next(cx) {
        break item;
      }
    })
  }
}

pub struct Source<'a> {
  sender: mpsc::UnboundedSender<DataValue<'a>>,
}

impl<'a> Source<'a> {
  fn new(sender: mpsc::UnboundedSender<DataValue<'a>>) -> Source<'a> {
    Source { sender }
  }

  pub async fn append(&mut self, data: Vec<DataValue<'a>>) -> Result<(), mpsc::SendError> {
    for item in &data {
      self.sender.send(item.clone()).await?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::{source, source_finite};
  use futures::StreamExt;

  #[test]
  fn sends_finite() {
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = source_finite(data);

    futures::executor::block_on(async {
      let values: Vec<_> = source.collect().await;
      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
        ]
      )
    })
  }

  #[test]
  fn sends_and_appends() {
    let (mut source, mut stream) = source();

    futures::executor::block_on(async {
      source
        .append(vec![
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
        ])
        .await
        .unwrap();

      assert_eq!(
        vec![stream.next().await.unwrap(), stream.next().await.unwrap()],
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
        ]
      );

      source
        .append(vec![
          DataValue::from_pairs(vec![("a", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 8.0.into())]),
        ])
        .await
        .unwrap();

      assert_eq!(
        vec![stream.next().await.unwrap(), stream.next().await.unwrap()],
        vec![
          DataValue::from_pairs(vec![("a", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 8.0.into())]),
        ]
      );
    });
  }
}

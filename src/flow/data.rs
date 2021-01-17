use crate::data::DataValue;
use futures::task::{Context, Poll};
use futures::Stream;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::rc::Rc;

pub type DataStream<'a> = Box<dyn Stream<Item = Option<DataValue<'a>>> + Unpin + 'a>;

struct SourceInner<T> {
  queues: Vec<VecDeque<T>>,
}

impl<T> SourceInner<T> {
  pub fn new() -> SourceInner<T> {
    SourceInner::default()
  }

  pub fn add(&mut self) {
    self.queues.push(VecDeque::new());
  }

  pub fn len(&self) -> usize {
    self.queues.len()
  }

  pub fn receive_from(&mut self, index: usize) -> Option<T> {
    self.queues[index].pop_front()
  }
}

impl<T: Clone> SourceInner<T> {
  pub fn send(&mut self, value: T) {
    for rx in &mut self.queues {
      rx.push_back(value.clone());
    }
  }
}

impl<T> Default for SourceInner<T> {
  fn default() -> Self {
    SourceInner { queues: Vec::new() }
  }
}

pub struct Source<T> {
  inner: Rc<RefCell<SourceInner<Option<T>>>>,
}

impl<T> Source<T> {
  pub fn new() -> Source<T> {
    Source::default()
  }
}

impl<T: Clone> Source<T> {
  pub fn link(&self) -> SourceNode<T> {
    let mut inner = self.inner.borrow_mut();
    inner.add();

    SourceNode {
      index: inner.len() - 1,
      inner: Rc::clone(&self.inner),
    }
  }

  pub fn send(&self, data: Vec<T>) {
    let mut inner = self.inner.borrow_mut();
    for value in data {
      inner.send(Some(value));
    }
    inner.send(None);
  }
}

impl<T> Default for Source<T> {
  fn default() -> Self {
    Source {
      inner: Rc::new(RefCell::new(SourceInner::new())),
    }
  }
}

pub struct SourceNode<T> {
  index: usize,
  inner: Rc<RefCell<SourceInner<Option<T>>>>,
}

impl<T> SourceNode<T> {
  fn receive(&self) -> Option<Option<T>> {
    self.inner.borrow_mut().receive_from(self.index)
  }
}

impl<T: Clone> Clone for SourceNode<T> {
  fn clone(&self) -> Self {
    let mut inner = self.inner.borrow_mut();
    inner.add();

    SourceNode {
      index: inner.len() - 1,
      inner: Rc::clone(&self.inner),
    }
  }
}

impl<T> Unpin for SourceNode<T> {}

impl<'a> Stream for SourceNode<DataValue<'a>> {
  type Item = Option<DataValue<'a>>;

  fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Some(value) = self.receive() {
        break Some(value);
      }
    })
  }
}

pub struct Chunks<S> {
  source: S,
}

impl<S, T> Chunks<S>
where
  S: Stream<Item = Option<T>>,
{
  pub fn new(source: S) -> Chunks<S> {
    Chunks { source }
  }
}

impl<S> Unpin for Chunks<S> {}

impl<S, T> Stream for Chunks<S>
where
  S: Stream<Item = Option<T>> + Unpin,
{
  type Item = T;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source)
      .poll_next(cx)
      .map(|value| value.flatten())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use futures::StreamExt;

  #[test]
  fn sends_chunk() {
    let source: Source<DataValue> = Source::new();
    let node = Chunks::new(source.link());

    source.send(vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ]);
    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;
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
  fn multiple_sends() {
    let source: Source<DataValue> = Source::new();
    let mut node = source.link();

    futures::executor::block_on(async {
      source.send(vec![
        DataValue::from_pairs(vec![("a", 2.0.into())]),
        DataValue::from_pairs(vec![("a", 4.0.into())]),
      ]);

      assert_eq!(
        vec![
          node.next().await.unwrap(),
          node.next().await.unwrap(),
          node.next().await.unwrap()
        ],
        vec![
          Some(DataValue::from_pairs(vec![("a", 2.0.into())])),
          Some(DataValue::from_pairs(vec![("a", 4.0.into())])),
          None
        ]
      );

      source.send(vec![
        DataValue::from_pairs(vec![("a", 6.0.into())]),
        DataValue::from_pairs(vec![("a", 8.0.into())]),
      ]);

      assert_eq!(
        vec![
          node.next().await.unwrap(),
          node.next().await.unwrap(),
          node.next().await.unwrap()
        ],
        vec![
          Some(DataValue::from_pairs(vec![("a", 6.0.into())])),
          Some(DataValue::from_pairs(vec![("a", 8.0.into())])),
          None
        ]
      );
    });
  }
}

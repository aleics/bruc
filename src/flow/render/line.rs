use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

#[derive(Debug, PartialEq)]
pub struct Line {
  points: Vec<(f32, f32)>,
  width: f32,
  height: f32,
}

impl Line {
  pub fn new(points: Vec<(f32, f32)>, width: f32, height: f32) -> Line {
    Line {
      points,
      width,
      height,
    }
  }

  pub fn points(&self) -> &Vec<(f32, f32)> {
    &self.points
  }

  pub fn width(&self) -> &f32 {
    &self.width
  }

  pub fn height(&self) -> &f32 {
    &self.height
  }
}

pub struct PointsNode<S> {
  x: S,
  y: S,
}

impl<S> PointsNode<S> {
  pub fn new(x: S, y: S) -> PointsNode<S> {
    PointsNode { x, y }
  }
}

impl<S> Stream for PointsNode<S>
where
  S: Stream<Item = f32> + Unpin,
{
  type Item = Vec<(f32, f32)>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut points = Vec::new();

    Poll::Ready(loop {
      let next_x = Pin::new(&mut self.x).poll_next(cx);
      let next_y = Pin::new(&mut self.y).poll_next(cx);

      if let (Poll::Ready(x), Poll::Ready(y)) = (next_x, next_y) {
        match (x, y) {
          (Some(x), Some(y)) => points.push((x, y)),
          (None, None) => break Some(points),
          _ => {}
        }
      };
    })
  }
}

pub struct LineNode<S>
where
  S: Stream<Item = f32> + Unpin,
{
  points: PointsNode<S>,
  width: S,
  height: S,
}

impl<S> LineNode<S>
where
  S: Stream<Item = f32> + Unpin,
{
  pub fn new(x: S, y: S, width: S, height: S) -> LineNode<S> {
    LineNode {
      points: PointsNode::new(x, y),
      width,
      height,
    }
  }
}

impl<S> Unpin for LineNode<S> where S: Stream<Item = f32> + Unpin {}

impl<S> Stream for LineNode<S>
where
  S: Stream<Item = f32> + Unpin,
{
  type Item = Line;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let next_points = Pin::new(&mut self.points).poll_next(cx);
    let next_width = Pin::new(&mut self.width).poll_next(cx);
    let next_height = Pin::new(&mut self.height).poll_next(cx);

    match (next_points, next_width, next_height) {
      (Poll::Ready(points), Poll::Ready(width), Poll::Ready(height)) => {
        match (points, width, height) {
          (Some(points), Some(width), Some(height)) => {
            Poll::Ready(Some(Line::new(points, width, height)))
          }
          _ => Poll::Ready(None),
        }
      }
      _ => Poll::Pending,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::flow::render::line::{Line, LineNode, PointsNode};
  use futures::{Stream, StreamExt};

  #[test]
  fn computes_points() {
    let mut node = PointsNode::new(numbers(vec![2.0, 4.0]), numbers(vec![5.0, 10.0]));
    futures::executor::block_on(async {
      assert_eq!(node.next().await.unwrap(), vec![(2.0, 5.0), (4.0, 10.0)]);
    });
  }

  #[test]
  fn computes_line() {
    let mut node = LineNode::new(
      numbers(vec![2.0]),
      numbers(vec![10.0]),
      numbers(vec![200.0]),
      numbers(vec![50.0]),
    );

    futures::executor::block_on(async {
      let line: Line = node.next().await.unwrap();
      assert_eq!(line, Line::new(vec![(2.0, 10.0)], 200.0, 50.0));
    });
  }

  fn numbers(values: Vec<f32>) -> impl Stream<Item = f32> {
    futures::stream::iter(values.into_iter())
  }
}

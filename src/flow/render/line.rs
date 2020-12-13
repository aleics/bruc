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

type NumberStream = Box<dyn Stream<Item = f32> + Unpin>;

pub struct PointsNode {
  x: NumberStream,
  y: NumberStream,
}

impl PointsNode {
  pub fn new(x: NumberStream, y: NumberStream) -> PointsNode {
    PointsNode { x, y }
  }
}

impl Stream for PointsNode {
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

pub struct LineNode {
  points: PointsNode,
  width: NumberStream,
  height: NumberStream,
}

impl LineNode {
  pub fn new() -> LineNode {
    LineNode::default()
  }

  pub fn with_points(&mut self, x: NumberStream, y: NumberStream) -> &mut LineNode {
    self.points = PointsNode::new(x, y);
    self
  }

  pub fn with_width(&mut self, width: NumberStream) -> &mut LineNode {
    self.width = width;
    self
  }

  pub fn with_height(&mut self, height: NumberStream) -> &mut LineNode {
    self.height = height;
    self
  }
}

impl Unpin for LineNode {}

impl Stream for LineNode {
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

impl Default for LineNode {
  fn default() -> Self {
    LineNode {
      points: PointsNode::new(
        Box::new(futures::stream::pending()),
        Box::new(futures::stream::pending()),
      ),
      width: Box::new(futures::stream::pending()),
      height: Box::new(futures::stream::pending()),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::flow::render::line::{Line, LineNode, NumberStream, PointsNode};
  use futures::StreamExt;

  #[test]
  fn computes_points() {
    let mut node = PointsNode::new(numbers(vec![2.0, 4.0]), numbers(vec![5.0, 10.0]));
    futures::executor::block_on(async {
      assert_eq!(node.next().await.unwrap(), vec![(2.0, 5.0), (4.0, 10.0)]);
    });
  }

  #[test]
  fn computes_line() {
    let mut node = LineNode::new();

    node.with_points(numbers(vec![2.0]), numbers(vec![10.0]));
    node.with_width(numbers(vec![200.0]));
    node.with_height(numbers(vec![50.0]));

    futures::executor::block_on(async {
      let line: Line = node.next().await.unwrap();
      assert_eq!(line, Line::new(vec![(2.0, 10.0)], 200.0, 50.0));
    });
  }

  fn numbers(values: Vec<f32>) -> NumberStream {
    Box::new(futures::stream::iter(values.into_iter()))
  }
}

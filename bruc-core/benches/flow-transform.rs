#![feature(test)]
extern crate test;

use bruc_core::data::DataValue;
use bruc_core::flow::data::{Chunks, Source};
use bruc_core::flow::transform::TransformNode;
use bruc_core::transform::filter::FilterPipe;
use bruc_core::transform::group::{GroupOperator, GroupPipe};
use bruc_core::transform::map::MapPipe;
use bruc_core::transform::pipe::Pipe;
use bruc_core::transform::Transform;
use bruc_expression::data::DataItem;
use futures::StreamExt;
use test::Bencher;

#[bench]
fn bench_filter_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let transform = vec![Pipe::Filter(
    FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      let source: Source<DataValue> = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_filter_pipe_20(b: &mut Bencher) {
  let data = vec![
    DataValue::from_pairs(vec![("a", DataItem::Number(1.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(2.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(3.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(4.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(5.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(6.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(7.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(8.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(9.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(10.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(11.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(12.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(13.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(14.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(15.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(16.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(17.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(18.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(19.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(20.0))]),
  ];
  let transform = vec![Pipe::Filter(
    FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_map_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let transform = vec![Pipe::Map(
    MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_map_pipe_20(b: &mut Bencher) {
  let data = vec![
    DataValue::from_pairs(vec![("a", DataItem::Number(1.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(2.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(3.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(4.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(5.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(6.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(7.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(8.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(9.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(10.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(11.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(12.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(13.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(14.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(15.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(16.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(17.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(18.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(19.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(20.0))]),
  ];
  let transform = vec![Pipe::Map(
    MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_group_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let transform = vec![Pipe::Group(GroupPipe::new(
    "a",
    GroupOperator::Count,
    "count",
  ))];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_group_pipe_20(b: &mut Bencher) {
  let data = vec![
    DataValue::from_pairs(vec![("a", DataItem::Number(1.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(2.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(3.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(4.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(5.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(6.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(7.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(8.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(9.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(10.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(11.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(12.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(13.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(14.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(15.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(16.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(17.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(18.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(19.0))]),
    DataValue::from_pairs(vec![("a", DataItem::Number(20.0))]),
  ];
  let transform = vec![Pipe::Group(GroupPipe::new(
    "a",
    GroupOperator::Count,
    "count",
  ))];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#[bench]
fn bench_10_pipes_2_10_vars_maps(b: &mut Bencher) {
  let transform = vec![
    Pipe::Map(MapPipe::new("(a + 1)", "k").unwrap()),
    Pipe::Map(MapPipe::new("(b + 2)", "l").unwrap()),
    Pipe::Map(MapPipe::new("(c + 3)", "m").unwrap()),
    Pipe::Map(MapPipe::new("(d + 4)", "n").unwrap()),
    Pipe::Map(MapPipe::new("(e + 5)", "o").unwrap()),
    Pipe::Map(MapPipe::new("(f + 6)", "p").unwrap()),
    Pipe::Map(MapPipe::new("(g + 7)", "q").unwrap()),
    Pipe::Map(MapPipe::new("(h + 8)", "r").unwrap()),
    Pipe::Map(MapPipe::new("(i + 9)", "s").unwrap()),
    Pipe::Map(MapPipe::new("(j + 10)", "t").unwrap()),
  ];
  let data = vec![
    DataValue::from_pairs(vec![
      ("a", DataItem::Number(1.0)),
      ("b", DataItem::Number(2.0)),
      ("c", DataItem::Number(3.0)),
      ("d", DataItem::Number(4.0)),
      ("e", DataItem::Number(5.0)),
      ("f", DataItem::Number(6.0)),
      ("g", DataItem::Number(7.0)),
      ("h", DataItem::Number(8.0)),
      ("i", DataItem::Number(9.0)),
      ("j", DataItem::Number(10.0)),
    ]),
    DataValue::from_pairs(vec![
      ("a", DataItem::Number(1.0)),
      ("b", DataItem::Number(2.0)),
      ("c", DataItem::Number(3.0)),
      ("d", DataItem::Number(4.0)),
      ("e", DataItem::Number(5.0)),
      ("f", DataItem::Number(6.0)),
      ("g", DataItem::Number(7.0)),
      ("h", DataItem::Number(8.0)),
      ("i", DataItem::Number(9.0)),
      ("j", DataItem::Number(10.0)),
    ]),
  ];

  b.iter(|| {
    futures::executor::block_on(async {
      let source = Source::new();

      let node = Chunks::new(TransformNode::node(source.link(), &transform));
      source.send(data.clone());

      node.collect::<Vec<_>>().await;
    });
  });
}

#![feature(test)]
extern crate test;

use bruc_core::data::DataValue;
use bruc_core::graph::node::Operator;
use bruc_core::graph::Graph;
use bruc_core::transform::filter::FilterPipe;
use bruc_core::transform::map::MapPipe;
use bruc_expression::data::DataItem;
use test::Bencher;

#[bench]
fn bench_filter_pipe_1(b: &mut Bencher) {
  let mut graph = Graph::new();

  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];

  let source = graph.add_node(Operator::source(data));

  graph.add(
    Operator::filter(FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap()),
    vec![source],
  );

  b.iter(|| {
    futures::executor::block_on(async {
      graph.evaluate().await;
    });
  });
}

#[bench]
fn bench_filter_pipe_20(b: &mut Bencher) {
  let mut graph = Graph::new();

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

  let source = graph.add_node(Operator::source(data));

  graph.add(
    Operator::filter(FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap()),
    vec![source],
  );

  b.iter(|| {
    futures::executor::block_on(async {
      graph.evaluate().await;
    });
  });
}

#[bench]
fn bench_map_pipe_1(b: &mut Bencher) {
  let mut graph = Graph::new();

  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];

  let source = graph.add_node(Operator::source(data));

  graph.add(
    Operator::map(MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap()),
    vec![source],
  );

  b.iter(|| {
    futures::executor::block_on(async {
      graph.evaluate().await;
    });
  });
}

#[bench]
fn bench_map_pipe_20(b: &mut Bencher) {
  let mut graph = Graph::new();

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
  let source = graph.add_node(Operator::source(data));

  graph.add(
          Operator::map(MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap()),
    vec![source],
  );

  b.iter(|| {
      futures::executor::block_on(async {
          graph.evaluate().await;
      });
  });
}

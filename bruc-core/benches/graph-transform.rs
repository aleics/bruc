#![feature(test)]
extern crate test;

use bruc_core::data::DataValue;
use bruc_core::graph::node::Operator;
use bruc_core::graph::{Graph, Pulse, PulseValue};
use bruc_core::transform::filter::FilterPipe;
use bruc_core::transform::group::{GroupOperator, GroupPipe};
use bruc_core::transform::map::MapPipe;
use bruc_expression::data::DataItem;
use test::Bencher;

#[bench]
fn bench_filter_pipe_1(b: &mut Bencher) {
  let pulse = Pulse::single(vec![PulseValue::Data(DataValue::from_pairs(vec![(
    "a",
    DataItem::Number(1.0),
  )]))]);

  let operator = Operator::filter(FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap());

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_filter_pipe_20(b: &mut Bencher) {
  let pulse = Pulse::single(vec![
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(2.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(3.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(4.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(5.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(6.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(7.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(8.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(9.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(10.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(11.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(12.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(13.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(14.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(15.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(16.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(17.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(18.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(19.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(20.0))])),
  ]);

  let operator = Operator::filter(FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap());

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_map_pipe_1(b: &mut Bencher) {
  let pulse = Pulse::single(vec![PulseValue::Data(DataValue::from_pairs(vec![(
    "a",
    DataItem::Number(1.0),
  )]))]);

  let operator = Operator::map(MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap());

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_map_pipe_20(b: &mut Bencher) {
  let pulse = Pulse::single(vec![
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(2.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(3.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(4.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(5.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(6.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(7.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(8.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(9.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(10.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(11.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(12.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(13.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(14.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(15.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(16.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(17.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(18.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(19.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(20.0))])),
  ]);

  let operator = Operator::map(MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap());

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_group_pipe_1(b: &mut Bencher) {
  let operator = Operator::group(GroupPipe::new("a", GroupOperator::Count, "count"));

  let pulse = Pulse::single(vec![PulseValue::Data(DataValue::from_pairs(vec![(
    "a",
    DataItem::Number(1.0),
  )]))]);

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_group_pipe_20(b: &mut Bencher) {
  let operator = Operator::group(GroupPipe::new("a", GroupOperator::Count, "count"));

  let pulse = Pulse::single(vec![
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(2.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(3.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(4.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(5.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(6.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(7.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(8.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(9.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(10.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(11.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(12.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(13.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(14.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(15.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(16.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(17.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(18.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(19.0))])),
    PulseValue::Data(DataValue::from_pairs(vec![("a", DataItem::Number(20.0))])),
  ]);

  b.iter(|| {
    futures::executor::block_on(async { operator.evaluate(pulse.clone()).await });
  });
}

#[bench]
fn bench_10_pipes_2_10_vars_maps(b: &mut Bencher) {
  let mut graph = Graph::new();

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

  let source = graph.add_node(Operator::data(data));

  let next = graph.add(
    Operator::map(MapPipe::new("(a + 1)", "k").unwrap()),
    vec![source],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(b + 2)", "l").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(c + 3)", "m").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(d + 4)", "n").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(e + 5)", "o").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(f + 6)", "p").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(g + 7)", "q").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(h + 8)", "r").unwrap()),
    vec![next],
  );
  let next = graph.add(
    Operator::map(MapPipe::new("(i + 9)", "s").unwrap()),
    vec![next],
  );
  graph.add(
    Operator::map(MapPipe::new("(j + 10)", "t").unwrap()),
    vec![next],
  );

  b.iter(|| {
    futures::executor::block_on(async {
      graph.evaluate().await;
    });
  });
}

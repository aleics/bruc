#![feature(test)]
extern crate test;

use bruc::data::DataValue;
use bruc::flow::transform::chain;
use bruc::transform::filter::FilterPipe;
use bruc::transform::group::{GroupPipe, Operation};
use bruc::transform::map::MapPipe;
use bruc::transform::pipe::Pipe;
use bruc_expreter::data::DataItem;
use futures::StreamExt;
use test::Bencher;

#[bench]
fn bench_filter_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let pipes = vec![Pipe::Filter(
    FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_filter_pipe_20_sequentially(b: &mut Bencher) {
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
  let pipes = vec![Pipe::Filter(
    FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_map_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let pipes = vec![Pipe::Map(
    MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_map_pipe_20_sequentially(b: &mut Bencher) {
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
  let pipes = vec![Pipe::Map(
    MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap(),
  )];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_group_pipe_1(b: &mut Bencher) {
  let data = vec![DataValue::from_pairs(vec![("a", DataItem::Number(1.0))])];
  let pipes = vec![Pipe::Group(GroupPipe::new("a", Operation::Count, "count"))];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_group_pipe_20_sequentially(b: &mut Bencher) {
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
  let pipes = vec![Pipe::Group(GroupPipe::new("a", Operation::Count, "count"))];

  b.iter(|| {
    futures::executor::block_on(async {
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

#[bench]
fn bench_10_pipes_2_10_vars_maps(b: &mut Bencher) {
  let pipes = vec![
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
      chain(&data, &pipes).collect::<Vec<DataValue>>().await;
    });
  });
}

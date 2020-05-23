#![feature(test)]
extern crate test;

use test::Bencher;

use ebooler::vars::{Variable, Variables};

use transformer::filter::FilterPipe;
use transformer::group::{GroupPipe, Operation};
use transformer::map::MapPipe;
use transformer::pipe::Pipe;
use transformer::Engine;

#[bench]
fn bench_filter_pipe_1(b: &mut Bencher) {
  let filter = FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap();
  let pipes = vec![Pipe::Filter(filter)];

  b.iter(|| {
    Engine::new(vec![Variables::from_pairs(vec![(
      "a",
      Variable::Number(1.0),
    )])])
    .run(&pipes)
  });
}

#[bench]
fn bench_filter_pipe_20_sequentially(b: &mut Bencher) {
  let filter = FilterPipe::new("(a > 1) && (a < 4) && (a != 3)").unwrap();
  let pipes = vec![Pipe::Filter(filter)];

  b.iter(|| {
    Engine::new(vec![
      Variables::from_pairs(vec![("a", Variable::Number(1.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(2.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(3.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(4.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(5.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(6.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(7.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(8.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(9.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(10.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(11.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(12.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(13.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(14.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(15.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(16.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(17.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(18.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(19.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(20.0))]),
    ])
    .run(&pipes)
  });
}

#[bench]
fn bench_map_pipe_1(b: &mut Bencher) {
  let map = MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap();
  let pipes = vec![Pipe::Map(map)];

  b.iter(|| {
    Engine::new(vec![Variables::from_pairs(vec![(
      "a",
      Variable::Number(1.0),
    )])])
    .run(&pipes)
  });
}

#[bench]
fn bench_map_pipe_20_sequentially(b: &mut Bencher) {
  let map = MapPipe::new("(a + 1) / (a * 4) - (a + 2)", "b").unwrap();
  let pipes = vec![Pipe::Map(map)];

  b.iter(|| {
    Engine::new(vec![
      Variables::from_pairs(vec![("a", Variable::Number(1.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(2.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(3.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(4.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(5.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(6.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(7.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(8.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(9.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(10.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(11.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(12.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(13.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(14.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(15.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(16.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(17.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(18.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(19.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(20.0))]),
    ])
    .run(&pipes)
  });
}

#[bench]
fn bench_group_pipe_1(b: &mut Bencher) {
  let group = GroupPipe::new("a", Operation::Count, "count");
  let pipes = vec![Pipe::Group(group)];

  b.iter(|| {
    Engine::new(vec![Variables::from_pairs(vec![(
      "a",
      Variable::Number(1.0),
    )])])
    .run(&pipes)
  });
}

#[bench]
fn bench_group_pipe_20_sequentially(b: &mut Bencher) {
  let group = GroupPipe::new("a", Operation::Count, "count");
  let pipes = vec![Pipe::Group(group)];

  b.iter(|| {
    Engine::new(vec![
      Variables::from_pairs(vec![("a", Variable::Number(1.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(2.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(3.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(4.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(5.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(6.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(7.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(8.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(9.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(10.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(11.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(12.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(13.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(14.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(15.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(16.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(17.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(18.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(19.0))]),
      Variables::from_pairs(vec![("a", Variable::Number(20.0))]),
    ])
    .run(&pipes)
  });
}

#![feature(test)]
extern crate test;

use test::Bencher;

use ebooler::vars::{Variable, Variables};

use transformer::filter::FilterPipe;
use transformer::group::{GroupPipe, Operation};
use transformer::iter::chain;
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

#[bench]
fn bench_unchained_10_10_maps(b: &mut Bencher) {
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

  b.iter(|| {
    Engine::new(vec![
      Variables::from_pairs(vec![
        ("a", Variable::Number(1.0)),
        ("b", Variable::Number(2.0)),
        ("c", Variable::Number(3.0)),
        ("d", Variable::Number(4.0)),
        ("e", Variable::Number(5.0)),
        ("f", Variable::Number(6.0)),
        ("g", Variable::Number(7.0)),
        ("h", Variable::Number(8.0)),
        ("i", Variable::Number(9.0)),
        ("j", Variable::Number(10.0)),
      ]),
      Variables::from_pairs(vec![
        ("a", Variable::Number(1.0)),
        ("b", Variable::Number(2.0)),
        ("c", Variable::Number(3.0)),
        ("d", Variable::Number(4.0)),
        ("e", Variable::Number(5.0)),
        ("f", Variable::Number(6.0)),
        ("g", Variable::Number(7.0)),
        ("h", Variable::Number(8.0)),
        ("i", Variable::Number(9.0)),
        ("j", Variable::Number(10.0)),
      ]),
    ])
    .run(&pipes)
  });
}

#[bench]
fn bench_chained_10_10_maps(b: &mut Bencher) {
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
    Variables::from_pairs(vec![
      ("a", Variable::Number(1.0)),
      ("b", Variable::Number(2.0)),
      ("c", Variable::Number(3.0)),
      ("d", Variable::Number(4.0)),
      ("e", Variable::Number(5.0)),
      ("f", Variable::Number(6.0)),
      ("g", Variable::Number(7.0)),
      ("h", Variable::Number(8.0)),
      ("i", Variable::Number(9.0)),
      ("j", Variable::Number(10.0)),
    ]),
    Variables::from_pairs(vec![
      ("a", Variable::Number(1.0)),
      ("b", Variable::Number(2.0)),
      ("c", Variable::Number(3.0)),
      ("d", Variable::Number(4.0)),
      ("e", Variable::Number(5.0)),
      ("f", Variable::Number(6.0)),
      ("g", Variable::Number(7.0)),
      ("h", Variable::Number(8.0)),
      ("i", Variable::Number(9.0)),
      ("j", Variable::Number(10.0)),
    ]),
  ];

  b.iter(|| chain(&data, &pipes).collect::<Vec<Variables>>());
}

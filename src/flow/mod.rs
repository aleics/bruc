pub mod data;
pub mod scale;
pub mod transform;

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::source;
  use crate::flow::transform::chain;
  use crate::transform::filter::FilterPipe;
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;
  use futures::StreamExt;

  #[test]
  fn chains_source_transform() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("x >= 2 && y < 5").unwrap()),
      Pipe::Map(MapPipe::new("x + 2", "z").unwrap()),
    ];
    let data = vec![
      DataValue::from_pairs(vec![("x", 1.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 3.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 7.0.into()), ("y", 4.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 3.0.into())]),
      DataValue::from_pairs(vec![("x", 1.5.into()), ("y", 7.0.into())]),
      DataValue::from_pairs(vec![("x", 2.0.into()), ("y", 9.0.into())]),
    ];

    futures::executor::block_on(async {
      let (mut sink, source) = source();
      let mut stream = chain(source, &pipes);

      sink.append(data).await.unwrap();
      assert_eq!(
        vec![
          stream.next().await.unwrap(),
          stream.next().await.unwrap(),
          stream.next().await.unwrap()
        ],
        vec![
          DataValue::from_pairs(vec![
            ("x", 3.0.into()),
            ("y", 1.0.into()),
            ("z", 5.0.into())
          ]),
          DataValue::from_pairs(vec![
            ("x", 7.0.into()),
            ("y", 4.0.into()),
            ("z", 9.0.into())
          ]),
          DataValue::from_pairs(vec![
            ("x", 5.0.into()),
            ("y", 3.0.into()),
            ("z", 7.0.into())
          ]),
        ]
      );
    });
  }
}

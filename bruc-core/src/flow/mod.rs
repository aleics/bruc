pub mod data;
pub mod render;
pub mod scale;
pub mod transform;

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::TransformNode;
  use crate::transform::filter::FilterPipe;
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;
  use futures::StreamExt;

  #[test]
  fn chains_source_transform() {
    let transform = vec![
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
      let source: Source<DataValue> = Source::new();
      let node = TransformNode::node(source.link(), &transform);

      source.send(data);
      assert_eq!(
        Chunks::new(node).collect::<Vec<_>>().await,
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
          ])
        ]
      );
    });
  }
}

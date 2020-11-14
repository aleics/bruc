#[cfg(test)]
mod tests {
  use bruc::transform::data::DataValue;
  use bruc::transform::filter::{FilterPipe, FilterPredicate};
  use bruc::transform::group::{GroupPipe, Operation};
  use bruc::transform::map::{MapPipe, MapPredicate};
  use bruc::transform::pipe::Pipe;
  use bruc::transform::Source;

  #[test]
  fn deserialize_value() {
    let data_value: DataValue = serde_json::from_str(r#"{ "a": 2.0 }"#).unwrap();
    assert_eq!(data_value, DataValue::from_pairs(vec![("a", 2.0.into())]));
  }

  #[test]
  fn deserialize_map() {
    let map = serde_json::from_str::<MapPipe>(r#"{ "fn": "a + 2.0", "output": "b" }"#).unwrap();

    assert_eq!(map.predicate(), &MapPredicate::new("a + 2.0").unwrap());
    assert_eq!(map.output(), "b");
  }

  #[test]
  fn deserialize_group() {
    let group = serde_json::from_str::<GroupPipe>(
      r#"{
      "by": "a",
      "op": "count",
      "output": "count_a"
     }"#,
    )
    .unwrap();

    assert_eq!(group.by(), "a");
    assert_eq!(group.op(), &Operation::Count);
    assert_eq!(group.output(), "count_a");
  }

  #[test]
  fn deserialize_filter() {
    let filter = serde_json::from_str::<FilterPipe>(r#""a > 2.0""#).unwrap();
    assert_eq!(
      filter.predicate(),
      &FilterPredicate::new("a > 2.0").unwrap()
    );
  }

  #[test]
  fn deserializes_pipes() {
    let pipes_json = r#"[
        { "filter": "a > 2" },
        { "map": { "fn": "a + 2", "output": "b" } },
        { "group": { "by": "b", "op": "count", "output": "count" } }
      ]"#;
    let pipes: Vec<Pipe> = serde_json::from_str(pipes_json).unwrap();

    assert_eq!(pipes.len(), 3);
  }

  #[test]
  fn deserializes_source() {
    let source_json = r#"{
      "data": [
        { "a": 2.0 },
        { "a": 4.0 },
        { "a": 6.0 }
      ],
      "pipes": [
        { "filter": "a > 2" },
        { "map": { "fn": "a + 2", "output": "b" } },
        { "group": { "by": "b", "op": "count", "output": "count" } }
       ]
    }"#;
    let source: Source = serde_json::from_str(source_json).unwrap();

    assert_eq!(
      source.data(),
      &vec![
        DataValue::from_pairs(vec![("a", 2.0.into())]),
        DataValue::from_pairs(vec![("a", 4.0.into())]),
        DataValue::from_pairs(vec![("a", 6.0.into())])
      ]
    );
    assert_eq!(
      source.pipes(),
      &vec![
        Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
        Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
        Pipe::Group(GroupPipe::new("b", Operation::Count, "count"))
      ]
    );
  }
}

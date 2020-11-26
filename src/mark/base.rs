use bruc_expreter::data::DataItem;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Phases<T> {
  update: Phase<T>,
}

impl<T> Phases<T> {
  pub fn new(props: T) -> Phases<T> {
    Phases {
      update: Phase { props },
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Phase<T> {
  #[cfg_attr(feature = "serde", serde(flatten))]
  props: T,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BaseMarkProperties<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  x: Option<DataSource<'a>>,
  #[cfg_attr(feature = "serde", serde(borrow))]
  y: Option<DataSource<'a>>,
  #[cfg_attr(feature = "serde", serde(borrow))]
  width: Option<DataSource<'a>>,
  #[cfg_attr(feature = "serde", serde(borrow))]
  height: Option<DataSource<'a>>,
}

impl<'a> BaseMarkProperties<'a> {
  pub fn new(
    x: Option<DataSource<'a>>,
    y: Option<DataSource<'a>>,
    width: Option<DataSource<'a>>,
    height: Option<DataSource<'a>>,
  ) -> BaseMarkProperties<'a> {
    BaseMarkProperties {
      x,
      y,
      width,
      height,
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataSource<'a> {
  FieldSource {
    field: &'a str,
    scale: Option<&'a str>,
  },
  ValueSource(DataItem),
}

impl<'a> DataSource<'a> {
  pub fn field(field: &'a str, scale: Option<&'a str>) -> DataSource<'a> {
    DataSource::FieldSource { field, scale }
  }

  pub fn value(item: DataItem) -> DataSource<'a> {
    DataSource::ValueSource(item)
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::mark::base::{BaseMarkProperties, DataSource, Phase};

  #[test]
  fn deserialize_update_phase() {
    let phase: Phase<BaseMarkProperties> = serde_json::from_str(
      r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" },
        "width": 100,
        "height": 100
      }"#,
    )
    .unwrap();

    assert_eq!(
      phase,
      Phase {
        props: BaseMarkProperties {
          x: Some(DataSource::FieldSource {
            field: "x",
            scale: Some("xscale"),
          }),
          y: Some(DataSource::FieldSource {
            field: "y",
            scale: Some("yscale"),
          }),
          width: Some(DataSource::ValueSource(100.0.into())),
          height: Some(DataSource::ValueSource(100.0.into())),
        }
      }
    );
  }

  #[test]
  fn deserialize_mark_props() {
    let mark_style: BaseMarkProperties = serde_json::from_str(
      r#"{
        "x": { "field": "x", "scale": "xscale" }
      }"#,
    )
    .unwrap();
    assert_eq!(
      mark_style,
      BaseMarkProperties {
        x: Some(DataSource::FieldSource {
          field: "x",
          scale: Some("xscale"),
        }),
        y: None,
        width: None,
        height: None,
      }
    );

    let mark_style: BaseMarkProperties = serde_json::from_str(r#"{ "y": 20 }"#).unwrap();
    assert_eq!(
      mark_style,
      BaseMarkProperties {
        x: None,
        y: Some(DataSource::ValueSource(20.0.into())),
        width: None,
        height: None,
      }
    );

    let mark_style: BaseMarkProperties = serde_json::from_str(
      r#"{
        "width": { "field": "x", "scale": "widthscale" }
      }"#,
    )
    .unwrap();
    assert_eq!(
      mark_style,
      BaseMarkProperties {
        x: None,
        y: None,
        width: Some(DataSource::FieldSource {
          field: "x",
          scale: Some("widthscale"),
        }),
        height: None,
      }
    );

    let mark_style: BaseMarkProperties = serde_json::from_str(r#"{ "height": 100 }"#).unwrap();
    assert_eq!(
      mark_style,
      BaseMarkProperties {
        x: None,
        y: None,
        width: None,
        height: Some(DataSource::ValueSource(100.0.into())),
      }
    );
  }

  #[test]
  fn deserialize_data_source() {
    let data_source: DataSource = serde_json::from_str(r#"{ "field": "x" }"#).unwrap();
    assert_eq!(
      data_source,
      DataSource::FieldSource {
        field: "x",
        scale: None,
      }
    );

    let data_source: DataSource =
      serde_json::from_str(r#"{ "field": "x", "scale": "horizontal" }"#).unwrap();
    assert_eq!(
      data_source,
      DataSource::FieldSource {
        field: "x",
        scale: Some("horizontal"),
      }
    );

    let data_source: DataSource = serde_json::from_str(r#"1"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(1.0.into()));

    let data_source: DataSource = serde_json::from_str(r#"true"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(true.into()));
  }
}

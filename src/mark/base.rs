use crate::mark::DataSource;
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
  use crate::mark::base::{BaseMarkProperties, Phase};
  use crate::mark::DataSource;

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
}

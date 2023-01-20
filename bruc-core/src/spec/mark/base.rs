use crate::spec::mark::DataSource;

pub(crate) const X_AXIS_FIELD_NAME: &str = "x";
pub(crate) const Y_AXIS_FIELD_NAME: &str = "y";
pub(crate) const WIDTH_FIELD_NAME: &str = "width";
pub(crate) const HEIGHT_FIELD_NAME: &str = "height";

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Phases<T> {
  pub(crate) update: Phase<T>,
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
  pub(crate) props: T,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BaseMarkProperties {
  pub(crate) x: Option<DataSource>,
  pub(crate) y: Option<DataSource>,
  pub(crate) width: Option<DataSource>,
  pub(crate) height: Option<DataSource>,
}

impl BaseMarkProperties {
  pub fn new(
    x: Option<DataSource>,
    y: Option<DataSource>,
    width: Option<DataSource>,
    height: Option<DataSource>,
  ) -> BaseMarkProperties {
    BaseMarkProperties {
      x,
      y,
      width,
      height,
    }
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::mark::base::{BaseMarkProperties, Phase};
  use crate::spec::mark::DataSource;

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
          x: Some(DataSource::field("x", Some("xscale"))),
          y: Some(DataSource::field("y", Some("yscale"))),
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
        x: Some(DataSource::field("x", Some("xscale"))),
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
        width: Some(DataSource::field("x", Some("widthscale"))),
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

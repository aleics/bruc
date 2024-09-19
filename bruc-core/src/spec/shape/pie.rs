use super::DataSource;

pub(crate) struct PiePropertiesBuilder {
  value: DataSource,
  pad_angle: Option<f32>,
  inner_radius: Option<f32>,
  outer_radius: Option<DataSource>,
}

impl PiePropertiesBuilder {
  pub(crate) fn new(value: DataSource) -> Self {
    PiePropertiesBuilder {
      value,
      pad_angle: None,
      inner_radius: None,
      outer_radius: None,
    }
  }

  pub(crate) fn with_pad_angle(mut self, pad_angle: f32) -> Self {
    self.pad_angle = Some(pad_angle);
    self
  }

  pub(crate) fn with_inner_radius(mut self, inner_radius: f32) -> Self {
    self.inner_radius = Some(inner_radius);
    self
  }

  pub(crate) fn with_outer_radius(mut self, outer_radius: DataSource) -> Self {
    self.outer_radius = Some(outer_radius);
    self
  }

  pub(crate) fn build(self) -> PieProperties {
    PieProperties {
      value: self.value,
      pad_angle: self.pad_angle.unwrap_or_default(),
      inner_radius: self.inner_radius.unwrap_or_default(),
      outer_radius: self.outer_radius,
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PieShape {
  #[cfg_attr(feature = "serde", serde(rename = "properties"))]
  pub(crate) props: PieProperties,
}

impl PieShape {
  pub(crate) fn new(props: PieProperties) -> Self {
    PieShape { props }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Deserialize),
  serde(rename_all = "camelCase")
)]
pub(crate) struct PieProperties {
  pub(crate) value: DataSource,
  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) pad_angle: f32,
  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) inner_radius: f32,
  #[cfg_attr(feature = "serde", serde(default))]
  // TODO: rename this to radius
  pub(crate) outer_radius: Option<DataSource>,
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::shape::{
    pie::{PieProperties, PiePropertiesBuilder, PieShape},
    DataSource,
  };

  #[test]
  fn serialize_minimal_pie_shape() {
    let pie_shape: PieShape = serde_json::from_str(
      r#"{
        "properties": {
          "value": { "field": "y" }
        }
      }"#,
    )
    .unwrap();

    assert_eq!(
      pie_shape,
      PieShape::new(PiePropertiesBuilder::new(DataSource::field("y", None)).build())
    )
  }

  #[test]
  fn serialize_pie_shape() {
    let pie_shape: PieShape = serde_json::from_str(
      r#"{
        "properties": {
          "value": { "field": "y" },
          "padAngle": 0.1,
          "innerRadius": 50
        }
      }"#,
    )
    .unwrap();

    assert_eq!(
      pie_shape,
      PieShape::new(
        PiePropertiesBuilder::new(DataSource::field("y", None))
          .with_pad_angle(0.1)
          .with_inner_radius(50.0)
          .build()
      )
    )
  }

  #[test]
  fn deserialize_props() {
    let props: PieProperties = serde_json::from_str(
      r#"{
        "value": { "field": "y" }
      }"#,
    )
    .unwrap();

    assert_eq!(
      props,
      PiePropertiesBuilder::new(DataSource::field("y", None)).build()
    );

    let props: PieProperties = serde_json::from_str(
      r#"{
        "value": { "field": "y" },
        "padAngle": 0.1,
        "innerRadius": 50
      }"#,
    )
    .unwrap();
    assert_eq!(
      props,
      PiePropertiesBuilder::new(DataSource::field("y", None))
        .with_pad_angle(0.1)
        .with_inner_radius(50.0)
        .build()
    );
  }
}

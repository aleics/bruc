use super::{base::BaseShapeProperties, DataSource};

pub(crate) struct BarPropertiesBuilder {
  width: Option<DataSource>,
  height: Option<DataSource>,
  x: Option<DataSource>,
  y: Option<DataSource>,
  fill: String,
}

impl BarPropertiesBuilder {
  pub(crate) fn new() -> Self {
    BarPropertiesBuilder {
      width: None,
      height: None,
      x: None,
      y: None,
      fill: default_fill(),
    }
  }

  pub(crate) fn with_width(mut self, width: DataSource) -> Self {
    self.width = Some(width);
    self
  }

  pub(crate) fn with_height(mut self, height: DataSource) -> Self {
    self.height = Some(height);
    self
  }

  pub(crate) fn with_x(mut self, x: DataSource) -> Self {
    self.x = Some(x);
    self
  }

  pub(crate) fn with_y(mut self, y: DataSource) -> Self {
    self.y = Some(y);
    self
  }

  pub(crate) fn with_fill(mut self, fill: &str) -> Self {
    self.fill = fill.to_string();
    self
  }

  pub(crate) fn build(self) -> BarProperties {
    BarProperties {
      base: BaseShapeProperties::new(self.x, self.y, self.width, self.height),
      fill: self.fill,
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BarShape {
  #[cfg_attr(feature = "serde", serde(rename = "properties"))]
  pub(crate) props: BarProperties,
}

impl BarShape {
  pub(crate) fn new(props: BarProperties) -> Self {
    BarShape { props }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Deserialize),
  serde(rename_all = "camelCase")
)]
pub(crate) struct BarProperties {
  #[cfg_attr(feature = "serde", serde(default = "default_fill"))]
  pub(crate) fill: String,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub(crate) base: BaseShapeProperties,
}

fn default_fill() -> String {
  "black".to_string()
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::shape::bar::{BarPropertiesBuilder, BarShape};
  use crate::spec::shape::DataSource;

  #[test]
  fn deserialize_bar_shape() {
    let bar_shape: BarShape = serde_json::from_str(
      r#"{
        "properties": {
          "x": { "field": "x", "scale": "xscale" },
          "width": 10.0,
          "height": { "field": "y", "scale": "yscale" }
        }
      }"#,
    )
    .unwrap();

    assert_eq!(
      bar_shape,
      BarShape::new(
        BarPropertiesBuilder::new()
          .with_x(DataSource::field("x", Some("xscale")))
          .with_width(DataSource::value(10.0.into()))
          .with_height(DataSource::field("y", Some("yscale")))
          .build()
      )
    )
  }
}

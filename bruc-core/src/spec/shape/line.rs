use crate::spec::shape::base::BaseShapeProperties;
use crate::spec::shape::DataSource;

pub(crate) struct LinePropertiesBuilder {
    x: Option<DataSource>,
    y: Option<DataSource>,
    interpolate: Interpolate,
    stroke: Option<String>,
    stroke_width: Option<f32>,
}

impl LinePropertiesBuilder {
    pub(crate) fn new() -> Self {
        LinePropertiesBuilder {
            x: None,
            y: None,
            interpolate: Interpolate::default(),
            stroke: None,
            stroke_width: None,
        }
    }

    pub(crate) fn with_x(mut self, x: DataSource) -> Self {
        self.x = Some(x);
        self
    }

    pub(crate) fn with_y(mut self, y: DataSource) -> Self {
        self.y = Some(y);
        self
    }

    pub(crate) fn with_interpolate(mut self, interpolate: Interpolate) -> Self {
        self.interpolate = interpolate;
        self
    }

    pub(crate) fn with_stroke(mut self, stroke: &str) -> Self {
        self.stroke = Some(stroke.to_string());
        self
    }

    pub(crate) fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = Some(stroke_width);
        self
    }

    pub(crate) fn build(self) -> LineProperties {
        LineProperties {
            base: BaseShapeProperties::new(self.x, self.y, None, None),
            interpolate: self.interpolate,
            stroke: self.stroke,
            stroke_width: self.stroke_width,
        }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LineShape {
    #[cfg_attr(feature = "serde", serde(rename = "properties"))]
    pub(crate) props: LineProperties,
}

impl LineShape {
    pub(crate) fn new(props: LineProperties) -> LineShape {
        LineShape { props }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub(crate) struct LineProperties {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) interpolate: Interpolate,
    pub(crate) stroke: Option<String>,
    pub(crate) stroke_width: Option<f32>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(crate) base: BaseShapeProperties,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Interpolate {
    #[default]
    Linear,
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use crate::spec::shape::line::{Interpolate, LineProperties, LinePropertiesBuilder, LineShape};
    use crate::spec::shape::DataSource;

    #[test]
    fn deserialize_line_shape() {
        let line_shape: LineShape = serde_json::from_str(
            r#"{
        "properties": {
          "x": { "field": "x", "scale": "xscale" },
          "y": { "field": "y", "scale": "yscale" },
          "interpolate": "linear",
          "strokeWidth": 2,
          "stroke": "red"
        }
      }"#,
        )
        .unwrap();

        assert_eq!(
            line_shape,
            LineShape::new(
                LinePropertiesBuilder::new()
                    .with_x(DataSource::field("x", Some("xscale")))
                    .with_y(DataSource::field("y", Some("yscale")))
                    .with_interpolate(Interpolate::Linear)
                    .with_stroke("red")
                    .with_stroke_width(2.0)
                    .build()
            )
        )
    }

    #[test]
    fn deserialize_props() {
        let props: LineProperties = serde_json::from_str(
            r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" }
      }"#,
        )
        .unwrap();
        assert_eq!(
            props,
            LinePropertiesBuilder::new()
                .with_x(DataSource::field("x", Some("xscale")))
                .with_y(DataSource::field("y", Some("yscale")))
                .build()
        );

        let props: LineProperties = serde_json::from_str(
            r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" },
        "interpolate": "linear"
      }"#,
        )
        .unwrap();
        assert_eq!(
            props,
            LinePropertiesBuilder::new()
                .with_x(DataSource::field("x", Some("xscale")))
                .with_y(DataSource::field("y", Some("yscale")))
                .build()
        );
    }
}

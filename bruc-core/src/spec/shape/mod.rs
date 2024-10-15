use crate::spec::shape::line::LineShape;
use bruc_expression::data::DataItem;
use pie::PieShape;
use point::PointShape;

use self::bar::BarShape;

pub(crate) mod bar;
pub(crate) mod base;
pub(crate) mod line;
pub(crate) mod pie;
pub(crate) mod point;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Shape {
    pub(crate) from: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(crate) kind: ShapeKind,
}

impl Shape {
    pub(crate) fn line(from: &str, line: LineShape) -> Self {
        Shape {
            from: from.to_string(),
            kind: ShapeKind::Line(line),
        }
    }

    pub(crate) fn bar(from: &str, bar: BarShape) -> Self {
        Shape {
            from: from.to_string(),
            kind: ShapeKind::Bar(bar),
        }
    }

    pub(crate) fn pie(from: &str, pie: PieShape) -> Self {
        Shape {
            from: from.to_string(),
            kind: ShapeKind::Pie(pie),
        }
    }

    pub(crate) fn point(from: &str, point: PointShape) -> Self {
        Shape {
            from: from.to_string(),
            kind: ShapeKind::Point(point),
        }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ShapeKind {
    Line(LineShape),
    Bar(BarShape),
    Pie(PieShape),
    Point(PointShape),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataSource {
    FieldSource {
        field: String,
        scale: Option<String>,
    },
    ValueSource(DataItem),
}

impl DataSource {
    pub fn field(field: &str, scale: Option<&str>) -> DataSource {
        DataSource::FieldSource {
            field: field.to_string(),
            scale: scale.map(|value| value.to_string()),
        }
    }

    pub fn value(item: DataItem) -> DataSource {
        DataSource::ValueSource(item)
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};
    use crate::spec::shape::{DataSource, Shape};

    #[test]
    fn deserialize_shape() {
        let shape: Shape = serde_json::from_str(
            r#"{
        "from": "table",
        "type": "line",
        "properties": {
          "x": { "field": "x", "scale": "xscale" },
          "y": { "field": "y", "scale": "yscale" }
        }
      }"#,
        )
        .unwrap();

        assert_eq!(
            shape,
            Shape::line(
                "table",
                LineShape::new(
                    LinePropertiesBuilder::new()
                        .with_x(DataSource::field("x", Some("xscale")))
                        .with_y(DataSource::field("y", Some("yscale")))
                        .build()
                )
            )
        );
    }

    #[test]
    fn deserialize_data_source() {
        let data_source: DataSource = serde_json::from_str(r#"{ "field": "x" }"#).unwrap();
        assert_eq!(data_source, DataSource::field("x", None));

        let data_source: DataSource =
            serde_json::from_str(r#"{ "field": "x", "scale": "horizontal" }"#).unwrap();
        assert_eq!(data_source, DataSource::field("x", Some("horizontal")));

        let data_source: DataSource = serde_json::from_str(r#"1"#).unwrap();
        assert_eq!(data_source, DataSource::ValueSource(1.0.into()));

        let data_source: DataSource = serde_json::from_str(r#"true"#).unwrap();
        assert_eq!(data_source, DataSource::ValueSource(true.into()));
    }
}

use super::DataSource;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PointShape {
    #[cfg_attr(feature = "serde", serde(rename = "properties"))]
    pub(crate) props: PointProperties,
}

impl PointShape {
    pub(crate) fn new(props: PointProperties) -> Self {
        PointShape { props }
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub(crate) struct PointProperties {
    pub(crate) color: Option<DataSource>,
    pub(crate) size: Option<DataSource>,
    pub(crate) x: Option<DataSource>,
    pub(crate) y: Option<DataSource>,
}

pub(crate) struct PointPropertiesBuilder {
    x: Option<DataSource>,
    y: Option<DataSource>,
    color: Option<DataSource>,
    size: Option<DataSource>,
}

impl PointPropertiesBuilder {
    pub(crate) fn new() -> Self {
        PointPropertiesBuilder {
            x: None,
            y: None,
            color: None,
            size: None,
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

    pub(crate) fn with_color(mut self, color: DataSource) -> Self {
        self.color = Some(color);
        self
    }

    pub(crate) fn with_size(mut self, size: DataSource) -> Self {
        self.size = Some(size);
        self
    }

    pub(crate) fn build(self) -> PointProperties {
        PointProperties {
            x: self.x,
            y: self.y,
            color: self.color,
            size: self.size,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use crate::spec::shape::{
        point::{PointProperties, PointPropertiesBuilder, PointShape},
        DataSource,
    };

    #[test]
    fn deserialize_point_shape() {
        let point_shape: PointShape = serde_json::from_str(
            r#"{
                "properties": {
                "x": { "field": "x", "scale": "xscale" },
                "y": { "field": "y", "scale": "yscale" },
                "color": "red",
                "size": { "field": "size" }
                }
            }"#,
        )
        .unwrap();

        assert_eq!(
            point_shape,
            PointShape::new(
                PointPropertiesBuilder::new()
                    .with_x(DataSource::field("x", Some("xscale")))
                    .with_y(DataSource::field("y", Some("yscale")))
                    .with_color(DataSource::value("red".into()))
                    .with_size(DataSource::field("size", None))
                    .build()
            )
        )
    }

    #[test]
    fn deserialize_props() {
        let props: PointProperties = serde_json::from_str(
            r#"{
                "x": { "field": "x", "scale": "xscale" },
                "y": { "field": "y", "scale": "yscale" }
            }"#,
        )
        .unwrap();
        assert_eq!(
            props,
            PointPropertiesBuilder::new()
                .with_x(DataSource::field("x", Some("xscale")))
                .with_y(DataSource::field("y", Some("yscale")))
                .build()
        );
    }
}

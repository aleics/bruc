use crate::spec::shape::DataSource;

pub(crate) const X_AXIS_FIELD_NAME: &str = "x";
pub(crate) const Y_AXIS_FIELD_NAME: &str = "y";
pub(crate) const WIDTH_FIELD_NAME: &str = "width";
pub(crate) const HEIGHT_FIELD_NAME: &str = "height";

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BaseShapeProperties {
    pub(crate) x: Option<DataSource>,
    pub(crate) y: Option<DataSource>,
    pub(crate) width: Option<DataSource>,
    pub(crate) height: Option<DataSource>,
}

impl BaseShapeProperties {
    pub fn new(
        x: Option<DataSource>,
        y: Option<DataSource>,
        width: Option<DataSource>,
        height: Option<DataSource>,
    ) -> BaseShapeProperties {
        BaseShapeProperties {
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
    use crate::spec::shape::base::BaseShapeProperties;
    use crate::spec::shape::DataSource;

    #[test]
    fn deserialize_update_phase() {
        let props: BaseShapeProperties = serde_json::from_str(
            r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" },
        "width": 100,
        "height": 100
      }"#,
        )
        .unwrap();

        assert_eq!(
            props,
            BaseShapeProperties {
                x: Some(DataSource::field("x", Some("xscale"))),
                y: Some(DataSource::field("y", Some("yscale"))),
                width: Some(DataSource::ValueSource(100.0.into())),
                height: Some(DataSource::ValueSource(100.0.into())),
            }
        );
    }

    #[test]
    fn deserialize_shape_props() {
        let shape_style: BaseShapeProperties = serde_json::from_str(
            r#"{
        "x": { "field": "x", "scale": "xscale" }
      }"#,
        )
        .unwrap();
        assert_eq!(
            shape_style,
            BaseShapeProperties {
                x: Some(DataSource::field("x", Some("xscale"))),
                y: None,
                width: None,
                height: None,
            }
        );

        let shape_style: BaseShapeProperties = serde_json::from_str(r#"{ "y": 20 }"#).unwrap();
        assert_eq!(
            shape_style,
            BaseShapeProperties {
                x: None,
                y: Some(DataSource::ValueSource(20.0.into())),
                width: None,
                height: None,
            }
        );

        let shape_style: BaseShapeProperties = serde_json::from_str(
            r#"{
        "width": { "field": "x", "scale": "widthscale" }
      }"#,
        )
        .unwrap();
        assert_eq!(
            shape_style,
            BaseShapeProperties {
                x: None,
                y: None,
                width: Some(DataSource::field("x", Some("widthscale"))),
                height: None,
            }
        );

        let shape_style: BaseShapeProperties =
            serde_json::from_str(r#"{ "height": 100 }"#).unwrap();
        assert_eq!(
            shape_style,
            BaseShapeProperties {
                x: None,
                y: None,
                width: None,
                height: Some(DataSource::ValueSource(100.0.into())),
            }
        );
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Axis {
    pub(crate) scale: String,
    pub(crate) orientation: AxisOrientation,
}

impl Axis {
    pub(crate) fn new(scale: &str, orientation: AxisOrientation) -> Self {
        Axis {
            scale: scale.to_string(),
            orientation,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum AxisOrientation {
    Top,
    Bottom,
    Left,
    Right,
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use crate::spec::axis::{Axis, AxisOrientation};

    #[test]
    fn deserialize_axis() {
        let axis: Axis = serde_json::from_str(
            r#"{
        "scale": "x",
        "orientation": "left"
      }"#,
        )
        .unwrap();

        assert_eq!(axis, Axis::new("x", AxisOrientation::Left));
    }
}

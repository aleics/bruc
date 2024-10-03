use crate::data::DataValue;
use crate::spec::transform::pipe::Pipe;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DataEntry {
    pub(crate) name: String,
    pub(crate) values: Vec<DataValue>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(crate) transform: Vec<Pipe>,
}

impl DataEntry {
    pub fn new(name: &str, values: Vec<DataValue>, transform: Vec<Pipe>) -> Self {
        DataEntry {
            name: name.to_string(),
            values,
            transform,
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use crate::spec::data::{DataEntry, DataValue};

    #[test]
    fn deserialize_data_value() {
        let data_value: DataValue = serde_json::from_str(r#"{ "a": 2.0 }"#).unwrap();
        assert_eq!(data_value, DataValue::from_pairs(vec![("a", 2.0.into())]));
    }

    #[test]
    fn deserializes_data() {
        let data: Vec<DataEntry> = serde_json::from_str(
            r#"[{
        "name": "my_data",
        "values": [{"a": 3.0, "b": true }]
      }]"#,
        )
        .unwrap();

        assert_eq!(
            data,
            vec![DataEntry::new(
                "my_data",
                vec![DataValue::from_pairs(vec![
                    ("a", 3.0.into()),
                    ("b", true.into())
                ])],
                Vec::new()
            )]
        );
    }
}

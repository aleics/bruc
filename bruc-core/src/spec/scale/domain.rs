#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Domain {
    Literal(Vec<f32>),
    DataField { data: String, field: String },
}

impl Domain {
    pub fn default_literal() -> Domain {
        Domain::Literal(vec![0.0, 1.0])
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use crate::spec::scale::domain::Domain;

    #[test]
    fn deserialize_domain_literal() {
        let domain: Domain = serde_json::from_str(r#"[0, 100]"#).unwrap();
        assert_eq!(domain, Domain::Literal(vec![0.0, 100.0]));
    }

    #[test]
    fn deserialize_domain_data_field() {
        let domain: Domain =
            serde_json::from_str(r#"{ "data": "primary", "field": "x" }"#).unwrap();
        assert_eq!(
            domain,
            Domain::DataField {
                data: "primary".to_string(),
                field: "x".to_string()
            }
        );
    }
}

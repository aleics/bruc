const DEFAULT_LITERAL: DomainLiteral = DomainLiteral {
  value: (0f32, 1f32),
};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Domain {
  Literal(DomainLiteral),
}

impl Domain {
  pub fn default_literal() -> Domain {
    Domain::Literal(DomainLiteral::default())
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DomainLiteral {
  value: (f32, f32),
}

impl DomainLiteral {
  pub fn new(value: (f32, f32)) -> DomainLiteral {
    DomainLiteral { value }
  }

  pub fn value(&self) -> &(f32, f32) {
    &self.value
  }
}

impl Default for DomainLiteral {
  fn default() -> Self {
    DEFAULT_LITERAL
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::scale::domain::{Domain, DomainLiteral};

  #[test]
  fn deserialize_domain() {
    let domain: Domain =
      serde_json::from_str(r#"{ "type": "literal", "value": [0, 100] }"#).unwrap();
    assert_eq!(domain, Domain::Literal(DomainLiteral::new((0.0, 100.0))));
  }
}

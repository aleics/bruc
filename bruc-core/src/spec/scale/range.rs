#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Range {
  Literal(f32, f32),
}

impl Range {
  pub fn default_literal() -> Range {
    Range::Literal(0.0, 1.0)
  }

  pub(crate) fn from(&self) -> f32 {
    match self {
      Range::Literal(from, _) => *from,
    }
  }

  pub(crate) fn to(&self) -> f32 {
    match self {
      Range::Literal(_, to) => *to,
    }
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::scale::range::Range;

  #[test]
  fn deserialize_range_literal() {
    let domain: Range = serde_json::from_str(r#"[0, 100]"#).unwrap();
    assert_eq!(domain, Range::Literal(0.0, 100.0));
  }
}

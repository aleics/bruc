use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::export::Formatter;
use serde::{de, Deserialize, Deserializer};

use crate::scale::domain::Domain;
use crate::scale::linear::LinearScale;
use crate::scale::Scale;

impl<'de: 'a, 'a> Deserialize<'de> for Scale<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct ScaleVisitor;

    impl<'a> Visitor<'a> for ScaleVisitor {
      type Value = Scale<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("enum Scale")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        if let Some(key) = map.next_key()? {
          match key {
            "linear" => Ok(Scale::Linear(map.next_value()?)),
            _ => Err(de::Error::unknown_variant(key, &["linear"])),
          }
        } else {
          Err(de::Error::custom("empty object"))
        }
      }
    }

    deserializer.deserialize_any(ScaleVisitor)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for LinearScale<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct LinearScaleVisitor;

    impl<'a> Visitor<'a> for LinearScaleVisitor {
      type Value = LinearScale<'a>;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("struct LinearScale")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut name = None;
        let mut domain = None;
        let mut range = None;

        while let Some(key) = map.next_key()? {
          match key {
            "name" => {
              if name.is_some() {
                return Err(de::Error::duplicate_field("name"));
              }
              name = map.next_value().ok();
            }
            "domain" => {
              if domain.is_some() {
                return Err(de::Error::duplicate_field("domain"));
              }
              match map.next_value() {
                Ok(value) => domain = Some(value),
                Err(err) => return Err(err),
              };
            }
            "range" => {
              if range.is_some() {
                return Err(de::Error::duplicate_field("range"));
              }
              match map.next_value() {
                Ok(value) => range = Some(value),
                Err(err) => return Err(err),
              };
            }
            key => return Err(de::Error::unknown_field(key, &["name", "domain", "range"])),
          }
        }

        let name = name.ok_or(de::Error::missing_field("name"))?;
        Ok(LinearScale::new(name, domain, range))
      }
    }

    deserializer.deserialize_any(LinearScaleVisitor)
  }
}

impl<'de> Deserialize<'de> for Domain {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct DomainVisitor;

    impl<'a> Visitor<'a> for DomainVisitor {
      type Value = Domain;

      fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("enum Domain")
      }

      fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        if let Some(key) = map.next_key()? {
          match key {
            "literal" => Ok(Domain::Literal(map.next_value()?)),
            _ => Err(de::Error::unknown_variant(key, &["literal"])),
          }
        } else {
          Err(de::Error::custom("empty object"))
        }
      }
    }

    deserializer.deserialize_any(DomainVisitor)
  }
}

#[cfg(test)]
mod tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::Scale;

  #[test]
  fn deserialize_scale() {
    let scale: Scale = serde_json::from_str(
      r#"{
        "linear": {
          "name": "y",
          "domain": {
            "literal": [0, 10]
          },
          "range": [0, 100]
        }
      }"#,
    )
    .unwrap();

    assert_eq!(
      scale,
      Scale::Linear(LinearScale::new(
        "y",
        Some(Domain::Literal((0.0, 10.0))),
        Some((0.0, 100.0))
      ))
    );
  }

  #[test]
  fn does_not_deserialize_scale() {
    let result = serde_json::from_str::<Scale>(
      r#"{
        "invalid": "invalid"
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("unknown variant `invalid`, expected `linear`"));

    let result = serde_json::from_str::<Scale>(r#"{}"#)
      .err()
      .unwrap()
      .to_string();

    assert!(result.contains("empty object"));
  }

  #[test]
  fn deserialize_linear_scale() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
        "name": "y",
        "domain": {
          "literal": [0, 10]
        },
        "range": [0, 100]
      }"#,
    )
    .unwrap();
    assert_eq!(
      linear_scale,
      LinearScale::new("y", Some(Domain::Literal((0.0, 10.0))), Some((0.0, 100.0)))
    );

    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
        "name": "y",
        "range": [0, 100]
      }"#,
    )
    .unwrap();
    assert_eq!(
      linear_scale,
      LinearScale::new("y", None, Some((0.0, 100.0)))
    );

    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
        "name": "y",
        "domain": {
          "literal": [0, 10]
        }
      }"#,
    )
    .unwrap();
    assert_eq!(
      linear_scale,
      LinearScale::new("y", Some(Domain::Literal((0.0, 10.0))), None)
    );
  }

  #[test]
  fn does_not_deserialize_linear_scale() {
    let result = serde_json::from_str::<LinearScale>(
      r#"{
        "domain": {
          "literal": [0, 10]
        },
        "range": [0, 100]
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("missing field `name`"));

    let result = serde_json::from_str::<LinearScale>(
      r#"{
        "name": "y",
        "name": "z"
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("duplicate field `name`"));

    let result = serde_json::from_str::<LinearScale>(
      r#"{
        "name": "y",
        "domain": {
          "literal": [0, 10]
        },
        "domain": {
          "literal": [0, 20]
        },
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("duplicate field `domain`"));

    let result = serde_json::from_str::<LinearScale>(
      r#"{
        "name": "y",
        "range": [0, 10],
        "range": [0, 20]
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("duplicate field `range`"));

    let result = serde_json::from_str::<LinearScale>(
      r#"{
        "name": "y",
        "domain": {
          "literal": [0, 10]
        },
        "range": [0, 100],
        "invalid": "invalid"
      }"#,
    )
    .err()
    .unwrap()
    .to_string();

    assert!(result.contains("unknown field `invalid`, expected one of `name`, `domain`, `range`"));
  }

  #[test]
  fn deserialize_domain() {
    let domain: Domain = serde_json::from_str(r#"{ "literal": [0, 1] }"#).unwrap();
    assert_eq!(domain, Domain::Literal((0.0, 1.0)));
  }

  #[test]
  fn does_not_deserialize_domain() {
    let result = serde_json::from_str::<Domain>("{}")
      .err()
      .unwrap()
      .to_string();
    assert!(result.contains("empty object"));

    let result = serde_json::from_str::<Domain>(r#"{ "invalid": "invalid" }"#)
      .err()
      .unwrap()
      .to_string();
    assert!(result.contains("unknown variant `invalid`, expected `literal`"));
  }
}

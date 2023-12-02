use bruc_expression::data::{DataItem, DataSource};

use crate::data::DataValue;

use crate::spec::scale::domain::Domain;

use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};

use super::util::{interpolate, normalize};

#[derive(Debug, PartialEq)]
pub struct DomainOperator {
  domain: Domain,
}

impl DomainOperator {
  pub(crate) fn new(domain: Domain) -> Self {
    DomainOperator { domain }
  }

  fn resolve_domain(&self, values: &[DataValue]) -> Option<(f32, f32)> {
    match &self.domain {
      Domain::Literal(min, max) => Some((*min, *max)),
      Domain::DataField { field, .. } => {
        if values.is_empty() {
          return None;
        }

        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;

        for value in values {
          let Some(value) = value.get_number(field).copied() else {
            break;
          };

          min = min.min(value);
          max = max.max(value);
        }

        Some((min, max))
      }
    }
  }

  fn apply(&self, pulse: &SinglePulse) -> Option<(f32, f32)> {
    let SinglePulse::Data(values) = pulse else {
      return None;
    };

    self.resolve_domain(values)
  }
}

impl Evaluation for DomainOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    if let Some((min, max)) = self.apply(&single) {
      Pulse::domain(min, max)
    } else {
      Pulse::data(Vec::new())
    }
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate())
  }
}

/// `LinearOperator` represents an operator of the graph, which linearly scales data values from a
/// certain `field` reference, and creates a new field in the defined `output` field.
#[derive(Debug, PartialEq)]
pub struct LinearOperator {
  range: (f32, f32),
  field: String,
  output: String,
}

impl LinearOperator {
  /// Create a new `LinearOperator` instance.
  pub(crate) fn new(range: (f32, f32), field: &str, output: &str) -> Self {
    LinearOperator {
      range,
      field: field.to_string(),
      output: output.to_string(),
    }
  }

  /// Apply the operator's logic by linearly scaling the referenced `field` and creating a new
  /// `output` field.
  fn apply(&self, values: &[DataValue], domain: (f32, f32)) -> Vec<DataValue> {
    let mut result = values.to_vec();

    // Iterate over the current series
    for value in &mut result {
      // Apply scale to field
      let scale_result = value
        .get_number(&self.field)
        .map(|value| interpolate(normalize(*value, domain), self.range));

      if let Some(scale_item) = scale_result {
        // Add scale result to value with the scale's name
        value.instance.clear();
        value.insert(&self.output, DataItem::Number(scale_item));
      }
    }

    result
  }
}

impl Evaluation for LinearOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    panic!("Linear operator requires a multi-pulse with data and a domain values.")
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let mut values = Vec::new();
    let mut domain: Option<(f32, f32)> = None;

    for pulse in multi.pulses {
      match pulse {
        SinglePulse::Data(data) => values.extend(data),
        SinglePulse::Domain(min, max) => domain = Some((min, max)),
        _ => continue,
      }
    }

    if values.is_empty() {
      return Pulse::data(Vec::new());
    }

    let domain = domain.expect("Domain pulse not provided for linear operator");

    Pulse::data(self.apply(&values, domain))
  }
}

/// `IdentityOperator` represents an operator of the graph, which copies a certain `field` reference
/// into an `output` field.
#[derive(Debug, PartialEq)]
pub struct IdentityOperator {
  field: String,
  output: String,
}

impl IdentityOperator {
  /// Create a new `IdentityOperator` instance.
  pub(crate) fn new(field: &str, output: &str) -> Self {
    IdentityOperator {
      field: field.to_string(),
      output: output.to_string(),
    }
  }

  /// Apply the operator's logic by copying the `field` value into a new `output` field.
  fn apply(&self, pulse: &SinglePulse) -> Vec<DataValue> {
    let SinglePulse::Data(values) = pulse else {
      return Vec::new();
    };
    let mut result = values.to_vec();

    // Iterate over the current series
    for value in &mut result {
      // Find field in data value
      if let Some(item) = value.get(&self.field) {
        // Add result to value with the output's name
        value.insert(&self.output, item.clone());
      }
    }

    result
  }
}

impl Evaluation for IdentityOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::data(self.apply(&single))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let values = multi.pulses.iter().fold(Vec::new(), |mut acc, pulse| {
      acc.extend(self.apply(pulse));
      acc
    });

    Pulse::data(values)
  }
}

#[cfg(test)]
mod tests {

  use futures::FutureExt;

  use crate::{
    data::DataValue,
    graph::{Evaluation, Pulse, SinglePulse},
    spec::scale::domain::Domain,
  };

  use super::{DomainOperator, LinearOperator};

  #[tokio::test]
  async fn domain_applies_for_literal() {
    let operator = DomainOperator::new(Domain::Literal(0.0, 5.0));
    let pulse = operator.evaluate(Pulse::data(vec![])).await;

    assert_eq!(pulse, Pulse::domain(0.0, 5.0))
  }

  #[tokio::test]
  async fn domain_applies_for_data_field() {
    let series = vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ];

    let operator = DomainOperator::new(Domain::DataField {
      data: "primary".to_string(),
      field: "a".to_string(),
    });
    let pulse = operator.evaluate(Pulse::data(series)).await;

    assert_eq!(pulse, Pulse::domain(-2.0, 15.0));
  }

  #[tokio::test]
  async fn domain_handles_empty_data() {
    let operator = DomainOperator::new(Domain::DataField {
      data: "primary".to_string(),
      field: "a".to_string(),
    });
    let pulse = operator.evaluate(Pulse::data(Vec::new())).await;

    assert_eq!(pulse, Pulse::data(Vec::new()));
  }

  #[tokio::test]
  async fn linear_panics_insufficient_pulse_values() {
    let series = vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ];

    let operator = LinearOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator.evaluate(Pulse::data(series)).catch_unwind().await;

    assert!(pulse.is_err());
  }

  #[tokio::test]
  async fn linear_applies_multi_pulse() {
    let first_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
    ]);
    let second_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ]);

    let domain = SinglePulse::Domain(0.0, 10.0);

    let operator = LinearOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator
      .evaluate(Pulse::multi(vec![first_pulse, second_pulse, domain]))
      .await;

    assert_eq!(
      pulse,
      Pulse::data(vec![
        DataValue::from_pairs(vec![("x", 0.0.into())]),
        DataValue::from_pairs(vec![("x", 0.5.into())]),
        DataValue::from_pairs(vec![("x", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 1.0.into())]),
      ])
    );
  }

  #[tokio::test]
  async fn linear_ignores_boolean_linear() {
    let data = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", true.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", false.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 1.0.into())]),
    ]);
    let domain = SinglePulse::Domain(0.0, 10.0);

    let operator = LinearOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator.evaluate(Pulse::multi(vec![data, domain])).await;

    assert_eq!(
      pulse,
      Pulse::data(vec![
        DataValue::from_pairs(vec![("a", true.into()), ("b", 1.0.into())]),
        DataValue::from_pairs(vec![("a", false.into()), ("b", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 0.2.into())]),
      ])
    );
  }

  #[tokio::test]
  async fn linear_handles_empty_data() {
    let operator = LinearOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator
      .evaluate(Pulse::multi(vec![SinglePulse::Data(vec![])]))
      .await;

    assert_eq!(pulse, Pulse::data(vec![]));
  }
}

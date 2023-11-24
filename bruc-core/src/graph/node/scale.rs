use bruc_expression::data::{DataItem, DataSource};

use crate::data::DataValue;

use crate::spec::scale::domain::Domain;

use crate::{
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  spec::scale::{linear::LinearScale, Scaler},
};


#[derive(Debug, PartialEq)]
pub struct DomainOperator {
  domain: Domain,
}

impl DomainOperator {
  pub(crate) fn new(domain: Domain) -> Self {
    DomainOperator { domain }
  }

  fn resolve_domain(&self, values: &[DataValue]) -> (f32, f32) {
    match &self.domain {
      Domain::Literal(min, max) => (*min, *max),
      Domain::DataField { field, .. } => {
        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;

        for value in values {
          let Some(value) = value.get_number(field).copied() else {
            break;
          };

          min = min.min(value);
          max = max.max(value);
        }

        (min, max)
      }
    }
  }

  fn apply(&self, pulse: &SinglePulse) -> Option<(f32, f32)> {
    let SinglePulse::Data(values) = pulse else {
      return None;
    };

    Some(self.resolve_domain(values))
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
  scale: LinearScale,
  field: String,
  output: String,
}

impl LinearOperator {
  /// Create a new `LinearOperator` instance.
  pub(crate) fn new(scale: LinearScale, field: &str, output: &str) -> Self {
    LinearOperator {
      scale,
      field: field.to_string(),
      output: output.to_string(),
    }
  }

  /// Apply the operator's logic by linearly scaling the referenced `field` and creating a new
  /// `output` field.
  fn apply(&self, pulse: &SinglePulse) -> Vec<DataValue> {
    let SinglePulse::Data(values) = pulse else {
      return Vec::new();
    };

    let mut result = values.to_vec();

    // Iterate over the current series
    for value in &mut result {
      // Apply scale to field
      let scale_result = value
        .get(&self.field)
        .and_then(|value| self.scale.scale(value));

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
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    let values = self.apply(&single);
    Pulse::data(values)
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let values = multi.pulses.iter().fold(Vec::new(), |mut acc, pulse| {
      acc.extend(self.apply(pulse));
      acc
    });

    Pulse::data(values)
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
  
  use crate::{
    data::DataValue,
    graph::{Evaluation, Pulse, SinglePulse},
    spec::scale::{domain::Domain, linear::LinearScale, range::Range},
  };

  use super::LinearOperator;

  #[tokio::test]
  async fn applies_linear_single_pulse() {
    let series = vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ];

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "a", "x");

    let pulse = operator.evaluate(Pulse::data(series)).await;

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
  async fn applies_linear_multi_pulse() {
    let first_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
    ]);
    let second_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ]);

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "a", "x");

    let pulse = operator
      .evaluate(Pulse::multi(vec![first_pulse, second_pulse]))
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
  async fn ignores_boolean_linear() {
    let series = vec![
      DataValue::from_pairs(vec![("a", true.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", false.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 1.0.into())]),
    ];

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "a", "x");

    let pulse = operator.evaluate(Pulse::data(series)).await;

    assert_eq!(
      pulse,
      Pulse::data(vec![
        DataValue::from_pairs(vec![("a", true.into()), ("b", 1.0.into())]),
        DataValue::from_pairs(vec![("a", false.into()), ("b", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 0.2.into())]),
      ])
    );
  }
}

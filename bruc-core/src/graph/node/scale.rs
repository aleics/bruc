use bruc_expression::data::{DataItem, DataSource};

use crate::data::DataValue;

use crate::graph::pulse::ResolvedDomain;
use crate::scale::Scale;
use crate::spec::scale::domain::Domain;

use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};
use crate::spec::scale::ScaleKind;

pub(crate) const SCALE_BAND_BANDWIDTH_FIELD_NAME: &str = "bandwidth";

#[derive(Debug, PartialEq)]
pub struct DomainIntervalOperator {
  domain: Domain,
}

impl DomainIntervalOperator {
  pub(crate) fn new(domain: Domain) -> Self {
    DomainIntervalOperator { domain }
  }

  fn resolve_domain(&self, values: &[DataValue]) -> Option<(f32, f32)> {
    match &self.domain {
      Domain::Literal(values) => Some((values[0], values[1])),
      Domain::DataField { field, .. } => {
        if values.is_empty() {
          return None;
        }

        let mut min: f32 = f32::MAX;
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

impl Evaluation for DomainIntervalOperator {
  async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    if let Some((min, max)) = self.apply(&single) {
      Pulse::domain(ResolvedDomain::Interval(min, max))
    } else {
      Pulse::data(Vec::new())
    }
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate()).await
  }
}

#[derive(Debug, PartialEq)]
pub struct DomainDiscreteOperator {
  domain: Domain,
  outer_padding: bool,
}

impl DomainDiscreteOperator {
  pub(crate) fn new(domain: Domain, outer_padding: bool) -> Self {
    DomainDiscreteOperator {
      domain,
      outer_padding,
    }
  }

  fn resolve_domain(&self, values: &[DataValue]) -> Vec<DataItem> {
    match &self.domain {
      Domain::Literal(values) => values
        .iter()
        .map(|value| DataItem::Number(*value))
        .collect(),
      Domain::DataField { field, .. } => values
        .iter()
        .flat_map(|value| value.get(field))
        .cloned()
        .collect(),
    }
  }

  fn apply(&self, pulse: &SinglePulse) -> Vec<DataItem> {
    let SinglePulse::Data(values) = pulse else {
      return Vec::new();
    };

    self.resolve_domain(values)
  }
}

impl Evaluation for DomainDiscreteOperator {
  async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::domain(ResolvedDomain::Discrete {
      values: self.apply(&single),
      outer_padding: self.outer_padding,
    })
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate()).await
  }
}

/// `LinearOperator` represents an operator of the graph, which linearly scales data values from a
/// certain `field` reference, and creates a new field in the defined `output` field.
#[derive(Debug, PartialEq)]
pub struct LinearOperator {
  scale: Scale,
  field: String,
  output: String,
}

impl LinearOperator {
  /// Create a new `LinearOperator` instance.
  pub(crate) fn new(range: (f32, f32), field: &str, output: &str) -> Self {
    LinearOperator {
      scale: Scale::linear(range),
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
        .map(|value| self.scale.apply(*value, domain));

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
  async fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    panic!("Linear operator requires a multi-pulse with data and a domain values.")
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let mut values = Vec::new();
    let mut domain: Option<(f32, f32)> = None;

    for pulse in multi.pulses {
      match pulse {
        SinglePulse::Data(data) => values.extend(data),
        SinglePulse::Domain(ResolvedDomain::Interval(min, max)) => domain = Some((min, max)),
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

#[derive(Debug, PartialEq)]
pub struct LogOperator {
  scale: Scale,
  field: String,
  output: String,
}

impl LogOperator {
  /// Create a new `LinearOperator` instance.
  pub(crate) fn new(range: (f32, f32), field: &str, output: &str) -> Self {
    LogOperator {
      scale: Scale::log(range),
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
        .map(|value| self.scale.apply(*value, domain));

      println!("scale result {:?} -> {:?}", value, scale_result);

      if let Some(scale_item) = scale_result {
        // Add scale result to value with the scale's name
        value.instance.clear();
        value.insert(&self.output, DataItem::Number(scale_item));
      }
    }

    result
  }
}

impl Evaluation for LogOperator {
  async fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    panic!("Linear operator requires a multi-pulse with data and a domain values.")
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let mut values = Vec::new();
    let mut domain: Option<(f32, f32)> = None;

    for pulse in multi.pulses {
      match pulse {
        SinglePulse::Data(data) => values.extend(data),
        SinglePulse::Domain(ResolvedDomain::Interval(min, max)) => domain = Some((min, max)),
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

/// `BandOperator` represents an operator of the graph, which maps a discrete domain to a
/// continuous range of values. `field` references the data source and `output` the name
/// of the new field with the result of the operator.
#[derive(Debug, PartialEq)]
pub struct BandOperator {
  range: (f32, f32),
  field: String,
  output: String,
}

impl BandOperator {
  /// Create a new `BandOperator` instance.
  pub(crate) fn new(range: (f32, f32), field: &str, output: &str) -> Self {
    BandOperator {
      range,
      field: field.to_string(),
      output: output.to_string(),
    }
  }

  // Apply the operator's logic to map the discrete domain into the range. The result is assigned
  // to a variable in the data value with the `output` name.
  fn apply(&self, values: &[DataValue], domain: (f32, f32)) -> Vec<DataValue> {
    let mut result = values.to_vec();

    let step = (self.range.1 - self.range.0) / (values.len() as f32);
    let padding = step / 4.0;
    let bandwidth = step - (padding * 2.0);
    let bandwidth_name = format!("{}_{}", &self.output, &SCALE_BAND_BANDWIDTH_FIELD_NAME);

    let range = (padding + self.range.0, padding + self.range.1 - step);
    let scale = Scale::linear(range);

    for value in &mut result {
      let scale_result = value
        .get(&self.field)
        .and_then(|value| value.get_number())
        .map(|value| scale.apply(*value, domain));

      if let Some(scale_item) = scale_result {
        // Add scale result to value with the scale's name
        value.instance.clear();
        value.insert(&self.output, DataItem::Number(scale_item));
        value.insert(&bandwidth_name, DataItem::Number(bandwidth));
      }
    }

    result
  }
}

impl Evaluation for BandOperator {
  async fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    panic!("Linear operator requires a multi-pulse with data and a domain values.")
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let mut values = Vec::new();
    let mut interval: Option<(f32, f32)> = None;

    for pulse in multi.pulses {
      match pulse {
        SinglePulse::Data(data) => values.extend(data),
        SinglePulse::Domain(domain) => interval = domain.interval(),
        _ => continue,
      }
    }

    let Some(domain) = interval else {
      return Pulse::data(values);
    };

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
  async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::data(self.apply(&single))
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
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
    graph::{node::scale::LogOperator, pulse::ResolvedDomain, Evaluation, Pulse, SinglePulse},
    spec::scale::domain::Domain,
  };

  use super::{BandOperator, DomainIntervalOperator, LinearOperator};

  #[tokio::test]
  async fn domain_applies_for_literal() {
    let operator = DomainIntervalOperator::new(Domain::Literal(vec![0.0, 5.0]));
    let pulse = operator.evaluate(Pulse::data(vec![])).await;

    assert_eq!(pulse, Pulse::domain(ResolvedDomain::Interval(0.0, 5.0)))
  }

  #[tokio::test]
  async fn domain_applies_for_data_field() {
    let series = vec![
      DataValue::from_pairs(vec![("a", (-2.0).into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 5.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 15.0.into()), ("b", 1.0.into())]),
    ];

    let operator = DomainIntervalOperator::new(Domain::DataField {
      data: "primary".to_string(),
      field: "a".to_string(),
    });
    let pulse = operator.evaluate(Pulse::data(series)).await;

    assert_eq!(pulse, Pulse::domain(ResolvedDomain::Interval(-2.0, 15.0)));
  }

  #[tokio::test]
  async fn domain_handles_empty_data() {
    let operator = DomainIntervalOperator::new(Domain::DataField {
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

    let domain = SinglePulse::Domain(ResolvedDomain::Interval(0.0, 10.0));

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
    let domain = SinglePulse::Domain(ResolvedDomain::Interval(0.0, 10.0));

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

  #[tokio::test]
  async fn band_applies_multi_pulse() {
    let first_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 0.0.into()), ("b", 5.0.into())]),
      DataValue::from_pairs(vec![("a", 1.0.into()), ("b", 2.0.into())]),
    ]);
    let second_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 1.0.into())]),
    ]);

    let domain = SinglePulse::Domain(ResolvedDomain::Discrete {
      values: vec![0.0.into(), 1.0.into(), 2.0.into(), 3.0.into()],
      outer_padding: true,
    });

    let operator = BandOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator
      .evaluate(Pulse::multi(vec![first_pulse, second_pulse, domain]))
      .await;

    assert_eq!(
      pulse,
      Pulse::data(vec![
        DataValue::from_pairs(vec![("x", 0.0625.into()), ("x_bandwidth", 0.125.into())]),
        DataValue::from_pairs(vec![("x", 0.3125.into()), ("x_bandwidth", 0.125.into())]),
        DataValue::from_pairs(vec![("x", 0.5625.into()), ("x_bandwidth", 0.125.into())]),
        DataValue::from_pairs(vec![("x", 0.8125.into()), ("x_bandwidth", 0.125.into())]),
      ])
    )
  }

  #[tokio::test]
  async fn band_handles_empty_domain() {
    let data = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 0.0.into()), ("b", 5.0.into())]),
      DataValue::from_pairs(vec![("a", 1.0.into()), ("b", 2.0.into())]),
    ]);
    let domain = SinglePulse::Domain(ResolvedDomain::Discrete {
      values: Vec::new(),
      outer_padding: false,
    });

    let operator = BandOperator::new((0.0, 1.0), "a", "x");
    let pulse = operator
      .evaluate(Pulse::multi(vec![data.clone(), domain]))
      .await;

    assert_eq!(pulse, Pulse::Single(data))
  }

  #[tokio::test]
  async fn log_applies_multi_pulse() {
    let first_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 100.0.into()), ("b", 1.0.into())]),
    ]);
    let second_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("a", 1000.0.into()), ("b", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 100000.0.into()), ("b", 1.0.into())]),
    ]);

    let domain = SinglePulse::Domain(ResolvedDomain::Interval(10.0, 100000.0));

    let operator = LogOperator::new((0.0, 600.0), "a", "x");
    let pulse = operator
      .evaluate(Pulse::multi(vec![first_pulse, second_pulse, domain]))
      .await;

    assert_eq!(
      pulse,
      Pulse::data(vec![
        DataValue::from_pairs(vec![("x", 0.0.into())]),
        DataValue::from_pairs(vec![("x", 150.0.into())]),
        DataValue::from_pairs(vec![("x", 300.0.into())]),
        DataValue::from_pairs(vec![("x", 600.0.into())]),
      ])
    );
  }
}

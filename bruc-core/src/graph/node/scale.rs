use bruc_expression::data::{DataItem, DataSource};

use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  scale::{linear::LinearScale, Scaler},
};

pub struct LinearOperator {
  scale: LinearScale,
  field: String,
  output: String,
}

impl LinearOperator {
  pub fn new(scale: LinearScale, field: &str, output: &str) -> Self {
    LinearOperator {
      scale,
      field: field.to_string(),
      output: output.to_string(),
    }
  }

  pub fn apply(&self, values: &Vec<DataValue>) -> Vec<DataValue> {
    let mut result = values.clone();

    // Iterate over the current series
    for value in &mut result {
      // Apply scale to field
      let scale_result = value
        .get(&self.field)
        .and_then(|value| self.scale.scale(value));

      if let Some(scale_item) = scale_result {
        // Add scale result to value with the scale's name
        value.insert(&self.output, DataItem::Number(scale_item));
      }
    }

    result
  }
}

impl Evaluation for LinearOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::single(self.apply(&single.values))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let values = multi.pulses.iter().fold(Vec::new(), |mut acc, pulse| {
      acc.extend(self.apply(&pulse.values));
      acc
    });

    Pulse::single(values)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    data::DataValue,
    graph::{Evaluation, Pulse, SinglePulse},
    scale::{domain::Domain, linear::LinearScale, range::Range},
  };

  use super::LinearOperator;

  #[tokio::test]
  async fn applies_linear_single_pulse() {
    let series = vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ];

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "x", "horizontal");

    let pulse = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![
        DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("horizontal", 0.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 0.5.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 1.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 1.0.into())
        ]),
      ])
    );
  }

  #[tokio::test]
  async fn applies_linear_multi_pulse() {
    let first_pulse = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
    ]);
    let second_pulse = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ]);

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "x", "horizontal");

    let pulse = operator
      .evaluate(Pulse::multi(vec![first_pulse, second_pulse]))
      .await;

    assert_eq!(
      pulse,
      Pulse::single(vec![
        DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("horizontal", 0.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 0.5.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 1.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 1.0.into())
        ]),
      ])
    );
  }

  #[tokio::test]
  async fn ignores_boolean_linear() {
    let series = vec![
      DataValue::from_pairs(vec![("x", true.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", false.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 2.0.into()), ("y", 1.0.into())]),
    ];

    let scale = LinearScale::new(Domain::Literal(0.0, 10.0), Range::Literal(0.0, 1.0));

    let operator = LinearOperator::new(scale, "x", "horizontal");

    let pulse = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![
        DataValue::from_pairs(vec![("x", true.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![("x", false.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![
          ("x", 2.0.into()),
          ("y", 1.0.into()),
          ("horizontal", 0.2.into())
        ]),
      ])
    );
  }
}

use std::{collections::HashMap, ops::AddAssign};

use bruc_expression::data::{DataItem, DataSource};

use crate::graph::PulseValue;
use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  spec::transform::{
    filter::FilterPipe,
    group::{GroupOperator as GroupOperatorSpec, GroupPipe},
    map::MapPipe,
  },
};

/// `MapOperator` represents an operator of the graph, which maps data values by a given map pipe.
#[derive(Debug, PartialEq)]
pub(crate) struct MapOperator {
  pipe: MapPipe,
}

impl MapOperator {
  /// Create a new `MapOperator` instance with a certain map pipe.
  pub(crate) fn new(pipe: MapPipe) -> Self {
    MapOperator { pipe }
  }

  /// Apply the operator's logic by executing the `MapPipe` to the incoming pulse values.
  fn apply(&self, values: &[PulseValue]) -> Vec<PulseValue> {
    let mut result = values.to_vec();

    for value in &mut result {
      if let PulseValue::Data(data_value) = value {
        self.pipe.apply(data_value);
      }
    }

    result
  }
}

impl Evaluation for MapOperator {
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

/// `FilterOperator` represents an operator of the graph, which filters out certain data values from
/// the incoming pulse data by applying a certain `FilterPipe`.
#[derive(Debug, PartialEq)]
pub struct FilterOperator {
  pipe: FilterPipe,
}

impl FilterOperator {
  /// Create a new `FilterOperator` instance with a certain filter pipe.
  pub(crate) fn new(pipe: FilterPipe) -> Self {
    FilterOperator { pipe }
  }

  /// Apply the operator's logic by executing the `FilterPipe` to the incoming pulse values.
  fn apply(&self, values: &[PulseValue]) -> Vec<PulseValue> {
    let mut result = Vec::with_capacity(values.len());

    for value in values {
      if let PulseValue::Data(data_value) = value {
        if self.pipe.apply(data_value) {
          result.push(value.clone());
        }
      }
    }

    result
  }
}

impl Evaluation for FilterOperator {
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

/// `GroupOperator` represents an operator of the graph, which applies a certain grouping logic to
/// incoming pulse values.
#[derive(Debug, PartialEq)]
pub enum GroupOperator {
  Count(CountOperator),
}

impl GroupOperator {
  /// Create a new `GroupOperator` logic defined by a certain `GroupPipe`.
  pub(crate) fn new(pipe: GroupPipe) -> Self {
    match pipe.op {
      GroupOperatorSpec::Count => GroupOperator::Count(CountOperator::new(pipe.by, pipe.output)),
    }
  }
}

impl Evaluation for GroupOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    match self {
      GroupOperator::Count(count) => count.evaluate_single(single),
    }
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    match self {
      GroupOperator::Count(count) => count.evaluate_multi(multi),
    }
  }
}

/// `CountOperator` represents a type of `GroupOperator`, which groups the incoming pulse data in
/// groups by counting the presence of a certain field's value in the incoming pulse values.
#[derive(Debug, PartialEq)]
pub struct CountOperator {
  by: String,
  output: String,
}

impl CountOperator {
  /// Create a new `CountOperator` instance for a certain `by` field reference and `output` field
  /// name.
  fn new(by: String, output: String) -> Self {
    CountOperator { by, output }
  }

  /// Apply the operator's logic by grouping the pulse data values in groups depending on the
  /// `by` field value.
  fn apply(&self, values: &[PulseValue]) -> Vec<PulseValue> {
    let mut counts: HashMap<DataItem, usize> = HashMap::new();

    for value in values {
      if let PulseValue::Data(data_value) = value {
        if let Some(target) = data_value.get(&self.by) {
          match counts.get_mut(target) {
            Some(count) => count.add_assign(1),
            None => {
              counts.insert(*target, 1);
            }
          }
        }
      }
    }

    let mut result = Vec::new();

    for (var, count) in counts {
      result.push(PulseValue::Data(DataValue::from_pairs(vec![
        (&self.by, var),
        (&self.output, DataItem::Number(count as f32)),
      ])));
    }

    result
  }
}

impl Evaluation for CountOperator {
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
  use crate::graph::PulseValue;
  use crate::spec::transform::group::GroupOperator as GroupOperatorSpec;
  use crate::{
    data::DataValue,
    graph::{
      node::transform::{FilterOperator, GroupOperator, MapOperator},
      Evaluation, Pulse, SinglePulse,
    },
    spec::transform::{filter::FilterPipe, group::GroupPipe, map::MapPipe},
  };

  #[tokio::test]
  async fn applies_map_single_pulse() {
    let series = vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", (-2.0).into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 5.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 10.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 15.0.into()),
        ("y", 1.0.into()),
      ])),
    ];

    let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("z", 0.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("z", 7.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("z", 12.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("z", 17.0.into())
        ])),
      ])
    );
  }

  #[tokio::test]
  async fn applies_map_multi_pulse() {
    let first = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", (-2.0).into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 5.0.into()),
        ("y", 1.0.into()),
      ])),
    ]);
    let second = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 10.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 15.0.into()),
        ("y", 1.0.into()),
      ])),
    ]);

    let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

    let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("z", 0.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("z", 7.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("z", 12.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("z", 17.0.into())
        ])),
      ])
    );
  }

  #[tokio::test]
  async fn applies_filter_single_pulse() {
    let series = vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", (-2.0).into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 5.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 10.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 15.0.into()),
        ("y", 1.0.into()),
      ])),
    ];

    let operator = FilterOperator::new(FilterPipe::new("x > y").unwrap());

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into())
        ])),
      ])
    );
  }

  #[tokio::test]
  async fn applies_filter_multi_pulse() {
    let first = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", (-2.0).into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 5.0.into()),
        ("y", 1.0.into()),
      ])),
    ]);
    let second = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 10.0.into()),
        ("y", 1.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 15.0.into()),
        ("y", 1.0.into()),
      ])),
    ]);

    let operator = FilterOperator::new(FilterPipe::new("x > y").unwrap());

    let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into())
        ])),
        PulseValue::Data(DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into())
        ])),
      ])
    );
  }

  #[tokio::test]
  async fn applies_group_single_pulse() {
    let series = vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("a", 1.0.into()),
        ("b", 2.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("a", 1.0.into()),
        ("b", 2.0.into()),
      ])),
    ];

    let operator = GroupOperator::new(GroupPipe::new("a", GroupOperatorSpec::Count, "count"));

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![PulseValue::Data(DataValue::from_pairs(vec![
        ("a", 1.0.into()),
        ("count", 2.0.into())
      ]))])
    );
  }
}

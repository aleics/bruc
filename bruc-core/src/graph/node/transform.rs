use std::{collections::HashMap, ops::AddAssign};

use bruc_expression::data::{DataItem, DataSource};

use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  transform::{
    filter::FilterPipe,
    group::{GroupOperator as GroupOperatorSpec, GroupPipe},
    map::MapPipe,
  },
};

#[derive(Debug)]
pub struct MapOperator {
  pipe: MapPipe,
}

impl MapOperator {
  pub fn new(pipe: MapPipe) -> Self {
    MapOperator { pipe }
  }

  fn apply(&self, values: &Vec<DataValue>) -> Vec<DataValue> {
    let mut result = values.clone();

    for value in &mut result {
      self.pipe.apply(value);
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

#[derive(Debug)]
pub struct FilterOperator {
  pipe: FilterPipe,
}

impl FilterOperator {
  pub fn new(pipe: FilterPipe) -> Self {
    FilterOperator { pipe }
  }

  fn apply(&self, values: &Vec<DataValue>) -> Vec<DataValue> {
    let mut result = Vec::with_capacity(values.len());

    for value in values {
      if self.pipe.apply(value) {
        result.push(value.clone())
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

pub enum GroupOperator {
  Count(CountOperator),
}

impl GroupOperator {
  pub fn new(pipe: GroupPipe) -> Self {
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

pub struct CountOperator {
  by: String,
  output: String,
}

impl CountOperator {
  fn new(by: String, output: String) -> Self {
    CountOperator { by, output }
  }

  fn apply(&self, values: &Vec<DataValue>) -> Vec<DataValue> {
    let mut counts: HashMap<DataItem, usize> = HashMap::new();

    for value in values {
      if let Some(target) = value.get(&self.by) {
        match counts.get_mut(target) {
          Some(count) => count.add_assign(1),
          None => {
            counts.insert(*target, 1);
          }
        }
      }
    }

    let mut result = Vec::new();

    for (var, count) in counts {
      result.push(DataValue::from_pairs(vec![
        (&self.by, var),
        (&self.output, DataItem::Number(count as f32)),
      ]))
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
  use crate::transform::group::GroupOperator as GroupOperatorSpec;
  use crate::{
    data::DataValue,
    graph::{
      node::transform::{FilterOperator, GroupOperator, MapOperator},
      Evaluation, Pulse, SinglePulse,
    },
    transform::{filter::FilterPipe, group::GroupPipe, map::MapPipe},
  };

  #[tokio::test]
  async fn applies_map_single_pulse() {
    let series = vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ];

    let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("z", 0.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("z", 7.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("z", 12.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("z", 17.0.into())
        ]),
      ])
    );
  }

  #[tokio::test]
  async fn applies_map_multi_pulse() {
    let first = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
    ]);
    let second = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ]);

    let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

    let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        DataValue::from_pairs(vec![
          ("x", (-2.0).into()),
          ("y", 1.0.into()),
          ("z", 0.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 5.0.into()),
          ("y", 1.0.into()),
          ("z", 7.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 10.0.into()),
          ("y", 1.0.into()),
          ("z", 12.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("x", 15.0.into()),
          ("y", 1.0.into()),
          ("z", 17.0.into())
        ]),
      ])
    );
  }

  #[tokio::test]
  async fn applies_filter_single_pulse() {
    let series = vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ];

    let operator = FilterOperator::new(FilterPipe::new("x > y").unwrap());

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
      ])
    );
  }

  #[tokio::test]
  async fn applies_filter_multi_pulse() {
    let first = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
    ]);
    let second = SinglePulse::new(vec![
      DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
    ]);

    let operator = FilterOperator::new(FilterPipe::new("x > y").unwrap());

    let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

    assert_eq!(
      result,
      Pulse::single(vec![
        DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
        DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
      ])
    );
  }

  #[tokio::test]
  async fn applies_group_single_pulse() {
    let series = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];

    let operator = GroupOperator::new(GroupPipe::new("a", GroupOperatorSpec::Count, "count"));

    let result = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      result,
      Pulse::single(vec![DataValue::from_pairs(vec![
        ("a", 2.0.into()),
        ("count", 2.0.into())
      ])])
    );
  }
}

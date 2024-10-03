use std::{collections::HashMap, ops::AddAssign};

use bruc_expression::data::{DataItem, DataSource};

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
pub struct MapOperator {
    pipe: MapPipe,
}

impl MapOperator {
    /// Create a new `MapOperator` instance with a certain map pipe.
    pub(crate) fn new(pipe: MapPipe) -> Self {
        MapOperator { pipe }
    }

    /// Apply the operator's logic by executing the `MapPipe` to the incoming pulse values.
    fn apply(&self, pulse: &SinglePulse) -> Vec<DataValue> {
        let SinglePulse::Data(values) = pulse else {
            return Vec::new();
        };

        let mut result = values.to_vec();
        for value in &mut result {
            self.pipe.apply(value);
        }
        result
    }
}

impl Evaluation for MapOperator {
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
    fn apply(&self, pulse: &SinglePulse) -> Vec<DataValue> {
        let SinglePulse::Data(values) = pulse else {
            return Vec::new();
        };

        values
            .iter()
            .filter(|value| self.pipe.apply(value))
            .cloned()
            .collect()
    }
}

impl Evaluation for FilterOperator {
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
            GroupOperatorSpec::Count => {
                GroupOperator::Count(CountOperator::new(pipe.by, pipe.output))
            }
        }
    }
}

impl Evaluation for GroupOperator {
    async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
        match self {
            GroupOperator::Count(count) => count.evaluate_single(single).await,
        }
    }

    async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
        match self {
            GroupOperator::Count(count) => count.evaluate_multi(multi).await,
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
    fn apply(&self, pulse: &SinglePulse) -> Vec<DataValue> {
        let SinglePulse::Data(values) = pulse else {
            return Vec::new();
        };
        let mut counts: HashMap<DataItem, usize> = HashMap::new();

        for value in values {
            if let Some(target) = value.get(&self.by) {
                match counts.get_mut(target) {
                    Some(count) => count.add_assign(1),
                    None => {
                        counts.insert(target.clone(), 1);
                    }
                }
            }
        }

        let mut result = Vec::new();

        for (var, count) in counts {
            result.push(DataValue::from_pairs(vec![
                (&self.by, var),
                (&self.output, DataItem::Number(count as f32)),
            ]));
        }

        result
    }
}

impl Evaluation for CountOperator {
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
            DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
        ];

        let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

        let result = operator.evaluate(Pulse::data(series)).await;

        assert_eq!(
            result,
            Pulse::data(vec![
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
        let first = SinglePulse::Data(vec![
            DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
        ]);
        let second = SinglePulse::Data(vec![
            DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
        ]);

        let operator = MapOperator::new(MapPipe::new("x + 2 * y", "z").unwrap());

        let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

        assert_eq!(
            result,
            Pulse::data(vec![
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

        let result = operator.evaluate(Pulse::data(series)).await;

        assert_eq!(
            result,
            Pulse::data(vec![
                DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
                DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
                DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
            ])
        );
    }

    #[tokio::test]
    async fn applies_filter_multi_pulse() {
        let first = SinglePulse::Data(vec![
            DataValue::from_pairs(vec![("x", (-2.0).into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
        ]);
        let second = SinglePulse::Data(vec![
            DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
            DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
        ]);

        let operator = FilterOperator::new(FilterPipe::new("x > y").unwrap());

        let result = operator.evaluate(Pulse::multi(vec![first, second])).await;

        assert_eq!(
            result,
            Pulse::data(vec![
                DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into())]),
                DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into())]),
                DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into())]),
            ])
        );
    }

    #[tokio::test]
    async fn applies_group_single_pulse() {
        let series = vec![
            DataValue::from_pairs(vec![("a", 1.0.into()), ("b", 2.0.into())]),
            DataValue::from_pairs(vec![("a", 1.0.into()), ("b", 2.0.into())]),
        ];

        let operator = GroupOperator::new(GroupPipe::new("a", GroupOperatorSpec::Count, "count"));

        let result = operator.evaluate(Pulse::data(series)).await;

        assert_eq!(
            result,
            Pulse::data(vec![DataValue::from_pairs(vec![
                ("a", 1.0.into()),
                ("count", 2.0.into())
            ])])
        );
    }
}

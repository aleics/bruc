use bruc_expression::data::DataItem;

use crate::{data::DataValue, scene::SceneItem};

/// `Pulse` represents the current state of a node in the graph for a certain evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Pulse {
  /// A single `Pulse` represents a single state.
  Single(SinglePulse),

  /// A multi `Pulse` represents multiple instances of a `Pulse` collected together.
  /// Multi pulses occur while evaluating nodes that have multiple source nodes connected to.
  Multi(MultiPulse),
}

impl Pulse {
  /// Create a new multi `Pulse` given a collection of single pulses
  pub(crate) fn multi(pulses: Vec<SinglePulse>) -> Self {
    Pulse::Multi(MultiPulse::new(pulses))
  }

  /// Create a new single `Pulse` with certain values.
  pub fn data(values: Vec<DataValue>) -> Self {
    Pulse::Single(SinglePulse::Data(values))
  }

  pub fn constant(value: DataValue) -> Self {
    Pulse::Single(SinglePulse::Constant(value))
  }

  pub fn shapes(values: Vec<SceneItem>) -> Self {
    Pulse::Single(SinglePulse::Shapes(values))
  }

  pub fn domain(domain: ResolvedDomain) -> Self {
    Pulse::Single(SinglePulse::Domain(domain))
  }

  /// Initialize an empty single `Pulse` instance.
  pub(crate) fn init() -> Self {
    Pulse::data(Vec::new())
  }

  /// Merge a collection of pulses together so that if more than one `SinglePulse` is found,
  /// a `MultiPulse` is created collecting all single pulses. Otherwise, a `SinglePulse` is
  /// returned.
  pub(crate) fn merge(pulses: Vec<Pulse>) -> Self {
    let mut single_pulses: Vec<SinglePulse> = pulses
      .into_iter()
      .flat_map(|pulse| match pulse {
        Pulse::Multi(multi) => multi.pulses,
        Pulse::Single(single) => vec![single],
      })
      .collect();

    if single_pulses.is_empty() {
      Pulse::data(Vec::new())
    } else if single_pulses.len() == 1 {
      let single_pulse = single_pulses.pop().unwrap();
      Pulse::Single(single_pulse)
    } else {
      Pulse::multi(single_pulses)
    }
  }
}

/// `SinglePulse` represents a type of `Pulse` with a single state instance, represented by a list
/// of values.
#[derive(Debug, Clone, PartialEq)]
pub enum SinglePulse {
  Constant(DataValue),
  Data(Vec<DataValue>),
  Shapes(Vec<SceneItem>),
  Domain(ResolvedDomain),
}

/// `MultiPulse` represents a type of `Pulse` with a number of `SinglePulse` instances.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiPulse {
  pub(crate) pulses: Vec<SinglePulse>,
}

impl MultiPulse {
  /// Create a new `MultiPulse` instance.
  pub fn new(pulses: Vec<SinglePulse>) -> Self {
    MultiPulse { pulses }
  }

  /// Aggregate the incoming multi pulse into a single pulse, by collecting all the needed data
  /// into a new single pulse.
  pub fn aggregate(&self) -> SinglePulse {
    let mut data_pairs = Vec::new();
    let mut constant_pairs = Vec::new();

    // Iterate through all the multi pulse instances and fold all the data values into
    // a new pulse value
    for single in &self.pulses {
      if let SinglePulse::Data(data_values) = single {
        // Extract all data values in pairs
        let data_values = data_values.iter().map(DataValue::pairs).collect();

        // Store the data values in the collected pulse values
        if data_pairs.is_empty() {
          data_pairs = data_values;
        } else {
          for j in 0..data_values.len() {
            if let Some(pairs) = data_pairs.get_mut(j) {
              pairs.extend(data_values.get(j).cloned().unwrap());
            }
          }
        }
      } else if let SinglePulse::Constant(value) = single {
        constant_pairs.extend(value.pairs())
      }
    }

    // Attach constant data values to all the data pairs
    for pairs in &mut data_pairs {
      pairs.extend(constant_pairs.clone());
    }

    // Create pulse values from the collected pairs
    let values = data_pairs
      .into_iter()
      .map(|pairs| DataValue::from_pairs(pairs))
      .collect();

    SinglePulse::Data(values)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedDomain {
  Interval(f32, f32),
  Discrete {
    values: Vec<DataItem>,
    outer_padding: bool,
  },
}

impl ResolvedDomain {
  pub(crate) fn interval(&self) -> Option<(f32, f32)> {
    match self {
      ResolvedDomain::Interval(min, max) => Some((*min, *max)),
      ResolvedDomain::Discrete { values, .. } => {
        if values.is_empty() {
          return None;
        }

        let mut domain = (f32::MAX, 0.0);

        for value in values {
          if let Some(num) = value.get_number().copied() {
            if num < domain.0 {
              domain.0 = num;
            }

            if num > domain.1 {
              domain.1 = num;
            }
          }
        }

        Some(domain)
      }
    }
  }
}

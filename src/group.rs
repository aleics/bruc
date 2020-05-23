use std::ops::AddAssign;

use ebooler::vars::{Variable, Variables};
use hashbrown::HashMap;
use serde::Deserialize;

use crate::pipe::Pipable;

#[derive(Deserialize, PartialEq, Debug)]
pub struct GroupPipe<'a> {
  #[serde(borrow)]
  by: &'a str,
  op: Operation,
  #[serde(borrow)]
  output: &'a str,
}

impl<'a> GroupPipe<'a> {
  pub fn new(by: &'a str, op: Operation, output: &'a str) -> GroupPipe<'a> {
    GroupPipe { by, op, output }
  }
}

impl<'a> Pipable<'a> for GroupPipe<'a> {
  fn transform(&self, data: &[Variables<'a>]) -> Vec<Variables<'a>> {
    let reps: HashMap<Variable, usize> =
      data
        .iter()
        .fold(HashMap::with_capacity(data.len()), |mut acc, item| {
          if let Some(target) = item.find(self.by) {
            if let Some(count) = acc.get_mut(target) {
              count.add_assign(1);
            } else {
              acc.insert(*target, 1);
            }
          }
          acc
        });

    reps
      .iter()
      .map(|(var, count)| {
        Variables::from_pairs(vec![
          (self.by, *var),
          (self.output, Variable::Number(*count as f32)),
        ])
      })
      .collect::<Vec<Variables>>()
  }
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Operation {
  #[serde(rename = "count")]
  Count,
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::group::{GroupPipe, Operation};
  use crate::pipe::Pipable;

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = vec![
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 2.0.into())]),
    ];
    let result = group.transform(&data);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &2.0.into());
    assert_eq!(result[0].find("count").unwrap(), &2.0.into());
  }

  #[test]
  fn finds_no_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = vec![
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("b", 3.0.into())]),
    ];
    let result = group.transform(&data);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &2.0.into());
    assert_eq!(result[0].find("count").unwrap(), &1.0.into());
  }

  #[test]
  fn deserialize() {
    let group = serde_json::from_str::<GroupPipe>(
      r#"{
      "by": "a",
      "op": "count",
      "output": "count_a"
     }"#,
    );

    assert!(group.is_ok());
  }
}

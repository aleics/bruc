use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::AddAssign;

use ebooler::vars::{Variable, Variables};
use serde::Deserialize;

use crate::pipe::{DataIterator, PipeIterator};

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

#[derive(Deserialize, PartialEq, Debug)]
pub enum Operation {
  #[serde(rename = "count")]
  Count,
}

pub struct GroupIterator<'a> {
  source: Box<DataIterator<'a>>,
}

impl<'a> GroupIterator<'a> {
  pub fn new(source: Box<DataIterator<'a>>) -> GroupIterator<'a> {
    GroupIterator { source }
  }

  #[inline]
  pub fn chain(source: PipeIterator<'a>, pipe: &'a GroupPipe<'a>) -> PipeIterator<'a> {
    let group_source = match pipe.op {
      Operation::Count => CountIterator::chain(source, pipe),
    };

    let iterator = GroupIterator::new(Box::new(group_source));
    PipeIterator::new(Box::new(iterator))
  }
}

impl<'a> Iterator for GroupIterator<'a> {
  type Item = Variables<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.source.next()
  }
}

struct CountIterator<'a> {
  source: IntoIter<Variable, usize>,
  by: &'a str,
  output: &'a str,
}

impl<'a> CountIterator<'a> {
  #[inline]
  fn new<I>(data: I, by: &'a str, output: &'a str) -> CountIterator<'a>
  where
    I: Iterator<Item = Variables<'a>>,
  {
    let reps = CountIterator::reps(data, by);
    CountIterator {
      source: reps.into_iter(),
      by,
      output,
    }
  }

  #[inline]
  fn reps<I>(data: I, by: &'a str) -> HashMap<Variable, usize>
  where
    I: Iterator<Item = Variables<'a>>,
  {
    data.fold(HashMap::new(), |mut acc, item| {
      if let Some(target) = item.find(by) {
        if let Some(count) = acc.get_mut(target) {
          count.add_assign(1);
        } else {
          acc.insert(*target, 1);
        }
      }
      acc
    })
  }

  #[inline]
  fn chain(source: PipeIterator<'a>, pipe: &'a GroupPipe<'a>) -> PipeIterator<'a> {
    let iterator = CountIterator::new(source, pipe.by, pipe.output);
    PipeIterator::new(Box::new(iterator))
  }
}

impl<'a> Iterator for CountIterator<'a> {
  type Item = Variables<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let (var, count) = self.source.next()?;
    let result = Variables::from_pairs(vec![
      (self.by, var),
      (self.output, Variable::Number(count as f32)),
    ]);

    Some(result)
  }
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::group::{GroupIterator, GroupPipe, Operation};
  use crate::pipe::PipeIterator;

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = [
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 2.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = GroupIterator::chain(source, &group);
    let result = iterator.collect::<Vec<Variables>>();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &2.0.into());
    assert_eq!(result[0].find("count").unwrap(), &2.0.into());
  }

  #[test]
  fn finds_no_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = [
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("b", 3.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = GroupIterator::chain(source, &group);
    let result = iterator.collect::<Vec<Variables>>();

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

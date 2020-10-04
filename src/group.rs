use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::AddAssign;

use crate::data::DataValue;
use crate::pipe::{DataIterator, PipeIterator};
use ebooler::data::DataItem;

#[derive(PartialEq, Debug)]
pub struct GroupPipe<'a> {
  by: &'a str,
  op: Operation,
  output: &'a str,
}

impl<'a> GroupPipe<'a> {
  pub fn new(by: &'a str, op: Operation, output: &'a str) -> GroupPipe<'a> {
    GroupPipe { by, op, output }
  }

  #[inline]
  pub fn by(&self) -> &'a str {
    &self.by
  }

  #[inline]
  pub fn op(&self) -> &Operation {
    &self.op
  }

  #[inline]
  pub fn output(&self) -> &'a str {
    &self.output
  }
}

#[derive(PartialEq, Debug)]
pub enum Operation {
  Count,
}

impl Operation {
  pub fn from_string(string: &str) -> Option<Operation> {
    match string {
      "count" => Some(Operation::Count),
      _ => None,
    }
  }
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
  type Item = DataValue<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.source.next()
  }
}

struct CountIterator<'a> {
  source: IntoIter<DataItem, usize>,
  by: &'a str,
  output: &'a str,
}

impl<'a> CountIterator<'a> {
  #[inline]
  fn new<I>(data: I, by: &'a str, output: &'a str) -> CountIterator<'a>
  where
    I: Iterator<Item = DataValue<'a>>,
  {
    let reps = CountIterator::reps(data, by);
    CountIterator {
      source: reps.into_iter(),
      by,
      output,
    }
  }

  #[inline]
  fn reps<I>(data: I, by: &'a str) -> HashMap<DataItem, usize>
  where
    I: Iterator<Item = DataValue<'a>>,
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
  type Item = DataValue<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let (var, count) = self.source.next()?;
    let result = DataValue::from_pairs(vec![
      (self.by, var),
      (self.output, DataItem::Number(count as f32)),
    ]);

    Some(result)
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::group::{GroupIterator, GroupPipe, Operation};
  use crate::pipe::PipeIterator;

  #[test]
  fn finds_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = GroupIterator::chain(source, &group);
    let result = iterator.collect::<Vec<DataValue>>();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &2.0.into());
    assert_eq!(result[0].find("count").unwrap(), &2.0.into());
  }

  #[test]
  fn finds_no_repetition() {
    let group = GroupPipe::new("a", Operation::Count, "count");
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("b", 3.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = GroupIterator::chain(source, &group);
    let result = iterator.collect::<Vec<DataValue>>();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &2.0.into());
    assert_eq!(result[0].find("count").unwrap(), &1.0.into());
  }
}

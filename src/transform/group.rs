#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::group::{GroupPipe, Operation};

  #[test]
  fn deserialize_group() {
    let group = serde_json::from_str::<GroupPipe>(
      r#"{
      "by": "a",
      "op": "count",
      "output": "count_a"
     }"#,
    )
    .unwrap();

    assert_eq!(group.by(), "a");
    assert_eq!(group.op(), &Operation::Count);
    assert_eq!(group.output(), "count_a");
  }
}

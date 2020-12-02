#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GroupPipe<'a> {
  by: &'a str,
  op: GroupOperator,
  output: &'a str,
}

impl<'a> GroupPipe<'a> {
  pub fn new(by: &'a str, op: GroupOperator, output: &'a str) -> GroupPipe<'a> {
    GroupPipe { by, op, output }
  }

  #[inline]
  pub fn by(&self) -> &'a str {
    &self.by
  }

  #[inline]
  pub fn op(&self) -> &GroupOperator {
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
pub enum GroupOperator {
  Count,
}

impl GroupOperator {
  pub fn from_string(string: &str) -> Option<GroupOperator> {
    match string {
      "count" => Some(GroupOperator::Count),
      _ => None,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::group::{GroupOperator, GroupPipe};

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
    assert_eq!(group.op(), &GroupOperator::Count);
    assert_eq!(group.output(), "count_a");
  }
}

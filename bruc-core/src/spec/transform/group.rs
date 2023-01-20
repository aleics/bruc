#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GroupPipe {
  pub(crate) by: String,
  pub(crate) op: GroupOperator,
  pub(crate) output: String,
}

impl GroupPipe {
  pub fn new(by: &str, op: GroupOperator, output: &str) -> GroupPipe {
    GroupPipe {
      by: by.to_string(),
      op,
      output: output.to_string(),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
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
  use crate::spec::transform::group::{GroupOperator, GroupPipe};

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

    assert_eq!(group.by, "a");
    assert_eq!(group.op, GroupOperator::Count);
    assert_eq!(group.output, "count_a");
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum Domain {
  Literal((f32, f32)),
}

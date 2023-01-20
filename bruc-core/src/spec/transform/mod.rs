use crate::spec::transform::pipe::Pipe;

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

pub type Transform = Vec<Pipe>;

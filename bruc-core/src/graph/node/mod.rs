use scale::LogOperator;
use shape::{PieOperator, PointOperator};

use crate::graph::node::scale::{IdentityOperator, LinearOperator};
use crate::graph::node::shape::{LineOperator, SceneWindow};
use crate::scale::Scale;
use crate::spec::axis::Axis;
use crate::spec::scale::domain::Domain;

use crate::spec::shape::bar::BarShape;
use crate::spec::shape::line::LineShape;
use crate::spec::shape::pie::PieShape;
use crate::{
    data::DataValue,
    spec::transform::{filter::FilterPipe, group::GroupPipe, map::MapPipe, pipe::Pipe},
};

use self::axis::AxisOperator;
use self::data::ConstantOperator;
use self::scale::{BandOperator, DomainIntervalOperator};
use self::shape::BarOperator;
use self::transform::{FilterOperator, MapOperator};
use self::{data::DataOperator, transform::GroupOperator};

use super::{Evaluation, Pulse};

pub(crate) mod axis;
pub(crate) mod color;
pub(crate) mod data;
pub(crate) mod scale;
pub(crate) mod shape;
pub(crate) mod transform;

/// `Node` represents a node in the `Graph` with a certain operator and a `Pulse` instance.
#[derive(Debug, PartialEq)]
pub struct Node {
    pub(crate) operator: Operator,
    pub(crate) pulse: Pulse,
}

impl Node {
    /// Initialize a new `Node` instance with a certain `Operator`. The associated pulse includes no
    /// data.
    pub(crate) fn init(operator: Operator) -> Self {
        Node {
            operator,
            pulse: Pulse::init(),
        }
    }

    /// Evaluate a `Pulse` instance passed to the node from its source. The resulting pulse is stored.
    pub(crate) async fn execute(&mut self, pulse: Pulse) {
        self.pulse = self.operator.evaluate(pulse).await;
    }
}

/// `Operator` collects all possible operators that can be used in a graph `Node`.
#[derive(Debug, PartialEq)]
pub enum Operator {
    Data(DataOperator),
    Constant(ConstantOperator),
    Map(MapOperator),
    Filter(FilterOperator),
    Group(GroupOperator),
    Line(LineOperator),
    Bar(BarOperator),
    Pie(PieOperator),
    Point(PointOperator),
    Axis(AxisOperator),
    DomainInterval(DomainIntervalOperator),
    Linear(LinearOperator),
    Log(LogOperator),
    Band(BandOperator),
    Identity(IdentityOperator),
}

impl Operator {
    /// Create a new data `Operator` instance.
    pub fn data(data: Vec<DataValue>) -> Self {
        Operator::Data(DataOperator::new(data))
    }

    pub fn constant(data: DataValue) -> Self {
        Operator::Constant(ConstantOperator::new(data))
    }

    /// Create a new `Operator` instance, given a transform `Pipe` definition.
    pub(crate) fn transform(pipe: Pipe) -> Self {
        match pipe {
            Pipe::Map(map) => Operator::map(map),
            Pipe::Filter(filter) => Operator::filter(filter),
            Pipe::Group(group) => Operator::group(group),
        }
    }

    /// Create a new map `Operator` instance.
    pub fn map(pipe: MapPipe) -> Self {
        Operator::Map(MapOperator::new(pipe))
    }

    /// Create a new filter `Operator` instance.
    pub fn filter(pipe: FilterPipe) -> Self {
        Operator::Filter(FilterOperator::new(pipe))
    }

    /// Create a new group `Operator` instance.
    pub fn group(pipe: GroupPipe) -> Self {
        Operator::Group(GroupOperator::new(pipe))
    }

    /// Create a new line `Operator` instance.
    pub(crate) fn line(shape: LineShape, window: SceneWindow) -> Self {
        Operator::Line(LineOperator::new(shape, window))
    }

    /// Create a new bar `Operator` instance
    pub(crate) fn bar(shape: BarShape, window: SceneWindow) -> Self {
        Operator::Bar(BarOperator::new(shape, window))
    }

    /// Create a new pie `Operator` instance
    pub(crate) fn pie(pie: PieShape, field: &str, window: SceneWindow) -> Operator {
        Operator::Pie(PieOperator::new(pie, field, window))
    }

    /// Create a new point `Operator` instance
    pub(crate) fn point(window: SceneWindow) -> Operator {
        Operator::Point(PointOperator::new(window))
    }

    /// Create a new axis `Operator` instance
    pub(crate) fn axis(axis: Axis, scale: Scale, window: SceneWindow) -> Self {
        Operator::Axis(AxisOperator::new(axis, scale, window))
    }

    /// Create a new identity `Operator` instance.
    pub(crate) fn identity(field: &str, output: &str) -> Self {
        Operator::Identity(IdentityOperator::new(field, output))
    }

    /// Create a new linear `Operator` instance for a certain `range`, with a given `field` reference and an `output`
    /// field name.
    pub(crate) fn linear(range: (f32, f32), field: &str, output: &str) -> Self {
        Operator::Linear(LinearOperator::new(range, field, output))
    }

    /// Create a new logarithmic `Operator` instance for a certain `range`, with a given `field` reference and an
    /// `output` field name.
    pub(crate) fn log(range: (f32, f32), field: &str, output: &str) -> Self {
        Operator::Log(LogOperator::new(range, field, output))
    }

    pub(crate) fn band(range: (f32, f32), field: &str, output: &str) -> Self {
        Operator::Band(BandOperator::new(range, field, output))
    }

    pub(crate) fn domain_interval(domain: Domain) -> Self {
        Operator::DomainInterval(DomainIntervalOperator::new(domain))
    }

    /// Evaluate the operator for a certain `Pulse`.
    pub async fn evaluate(&self, pulse: Pulse) -> Pulse {
        match self {
            Operator::Data(data) => data.evaluate(pulse).await,
            Operator::Constant(constant) => constant.evaluate(pulse).await,
            Operator::Map(map) => map.evaluate(pulse).await,
            Operator::Filter(filter) => filter.evaluate(pulse).await,
            Operator::Group(group) => group.evaluate(pulse).await,
            Operator::Line(line) => line.evaluate(pulse).await,
            Operator::Bar(bar) => bar.evaluate(pulse).await,
            Operator::Pie(pie) => pie.evaluate(pulse).await,
            Operator::Point(point) => point.evaluate(pulse).await,
            Operator::Axis(axis) => axis.evaluate(pulse).await,
            Operator::DomainInterval(domain_interval) => domain_interval.evaluate(pulse).await,
            Operator::Linear(linear) => linear.evaluate(pulse).await,
            Operator::Log(log) => log.evaluate(pulse).await,
            Operator::Band(band) => band.evaluate(pulse).await,
            Operator::Identity(identity) => identity.evaluate(pulse).await,
        }
    }
}

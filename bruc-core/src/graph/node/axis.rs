use crate::{
    graph::{pulse::ResolvedDomain, Evaluation, MultiPulse, Pulse, SinglePulse},
    scale::{Scale, ScaleTick},
    scene::{SceneAxisRule, SceneAxisTick, SceneItem},
    spec::axis::{Axis, AxisOrientation},
};

use super::shape::SceneWindow;

#[derive(Debug, PartialEq)]
pub struct AxisOperator {
    axis: Axis,
    scale: Scale,
    window: SceneWindow,
}

impl AxisOperator {
    pub(crate) fn new(axis: Axis, scale: Scale, window: SceneWindow) -> Self {
        AxisOperator {
            axis,
            scale,
            window,
        }
    }

    fn apply_interval(&self, domain: (f32, f32)) -> SinglePulse {
        let scene_item = self.create_axis(domain);
        SinglePulse::Shapes(vec![scene_item])
    }

    fn create_axis(&self, domain: (f32, f32)) -> SceneItem {
        let ticks = self.scale.ticks(domain);
        SceneItem::axis(
            self.create_ruler(),
            self.create_ticks(ticks),
            self.axis.orientation,
        )
    }

    fn create_ticks(&self, ticks: Vec<ScaleTick>) -> Vec<SceneAxisTick> {
        ticks
            .into_iter()
            .map(|tick| SceneAxisTick {
                position: self.orientation_position(tick.position),
                label: tick.label,
            })
            .collect()
    }

    fn create_ruler(&self) -> SceneAxisRule {
        let (from, to) = self.scale.range();

        SceneAxisRule {
            from: self.orientation_position(from),
            to: self.orientation_position(to),
        }
    }

    fn orientation_position(&self, position: f32) -> (f32, f32) {
        match self.axis.orientation {
            AxisOrientation::Top => (position, self.window.height),
            AxisOrientation::Bottom => (position, 0.0),
            AxisOrientation::Left => (0.0, position),
            AxisOrientation::Right => (self.window.width, position),
        }
    }
}

impl Evaluation for AxisOperator {
    async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
        match single {
            SinglePulse::Domain(domain) => {
                let Some(interval) = domain.interval() else {
                    return Pulse::shapes(Vec::new());
                };

                let pulse = match domain {
                    ResolvedDomain::Interval(_, _) => self.apply_interval(interval),
                };

                Pulse::Single(pulse)
            }
            _ => Pulse::shapes(Vec::new()),
        }
    }

    async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
        for pulse in multi.pulses {
            if let SinglePulse::Domain(ResolvedDomain::Interval(min, max)) = pulse {
                return Pulse::Single(self.apply_interval((min, max)));
            }
        }
        Pulse::shapes(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{
            node::{axis::AxisOperator, shape::SceneWindow},
            pulse::ResolvedDomain,
            Evaluation, Pulse,
        },
        scale::Scale,
        scene::{SceneAxisRule, SceneAxisTick, SceneItem},
        spec::axis::{Axis, AxisOrientation},
    };

    #[tokio::test]
    async fn creates_top_axis() {
        let operator = AxisOperator::new(
            Axis::new("horizontal", AxisOrientation::Top),
            Scale::linear((0.0, 200.0)),
            SceneWindow::new(200, 100),
        );

        let pulse = operator
            .evaluate(Pulse::domain(ResolvedDomain::Interval(0.0, 100.0)))
            .await;

        assert_eq!(
            pulse,
            Pulse::shapes(vec![SceneItem::axis(
                SceneAxisRule {
                    from: (0.0, 100.0),
                    to: (200.0, 100.0)
                },
                vec![
                    SceneAxisTick {
                        position: (0.0, 100.0),
                        label: Some("0.00".to_string())
                    },
                    SceneAxisTick {
                        position: (20.0, 100.0),
                        label: Some("10.00".to_string())
                    },
                    SceneAxisTick {
                        position: (40.0, 100.0),
                        label: Some("20.00".to_string())
                    },
                    SceneAxisTick {
                        position: (60.000004, 100.0),
                        label: Some("30.00".to_string())
                    },
                    SceneAxisTick {
                        position: (80.0, 100.0),
                        label: Some("40.00".to_string())
                    },
                    SceneAxisTick {
                        position: (100.0, 100.0),
                        label: Some("50.00".to_string())
                    },
                    SceneAxisTick {
                        position: (120.00001, 100.0),
                        label: Some("60.00".to_string())
                    },
                    SceneAxisTick {
                        position: (140.0, 100.0),
                        label: Some("70.00".to_string())
                    },
                    SceneAxisTick {
                        position: (160.0, 100.0),
                        label: Some("80.00".to_string())
                    },
                    SceneAxisTick {
                        position: (180.0, 100.0),
                        label: Some("90.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 100.0),
                        label: Some("100.00".to_string())
                    }
                ],
                AxisOrientation::Top
            )])
        )
    }

    #[tokio::test]
    async fn creates_bottom_axis() {
        let operator = AxisOperator::new(
            Axis::new("horizontal", AxisOrientation::Bottom),
            Scale::linear((0.0, 200.0)),
            SceneWindow::new(200, 100),
        );

        let pulse = operator
            .evaluate(Pulse::domain(ResolvedDomain::Interval(0.0, 100.0)))
            .await;

        assert_eq!(
            pulse,
            Pulse::shapes(vec![SceneItem::axis(
                SceneAxisRule {
                    from: (0.0, 0.0),
                    to: (200.0, 0.0)
                },
                vec![
                    SceneAxisTick {
                        position: (0.0, 0.0),
                        label: Some("0.00".to_string())
                    },
                    SceneAxisTick {
                        position: (20.0, 0.0),
                        label: Some("10.00".to_string())
                    },
                    SceneAxisTick {
                        position: (40.0, 0.0),
                        label: Some("20.00".to_string())
                    },
                    SceneAxisTick {
                        position: (60.000004, 0.0),
                        label: Some("30.00".to_string())
                    },
                    SceneAxisTick {
                        position: (80.0, 0.0),
                        label: Some("40.00".to_string())
                    },
                    SceneAxisTick {
                        position: (100.0, 0.0),
                        label: Some("50.00".to_string())
                    },
                    SceneAxisTick {
                        position: (120.00001, 0.0),
                        label: Some("60.00".to_string())
                    },
                    SceneAxisTick {
                        position: (140.0, 0.0),
                        label: Some("70.00".to_string())
                    },
                    SceneAxisTick {
                        position: (160.0, 0.0),
                        label: Some("80.00".to_string())
                    },
                    SceneAxisTick {
                        position: (180.0, 0.0),
                        label: Some("90.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 0.0),
                        label: Some("100.00".to_string())
                    }
                ],
                AxisOrientation::Bottom
            )])
        )
    }

    #[tokio::test]
    async fn creates_left_axis() {
        let operator = AxisOperator::new(
            Axis::new("vertical", AxisOrientation::Left),
            Scale::linear((0.0, 200.0)),
            SceneWindow::new(200, 100),
        );

        let pulse = operator
            .evaluate(Pulse::domain(ResolvedDomain::Interval(0.0, 100.0)))
            .await;

        assert_eq!(
            pulse,
            Pulse::shapes(vec![SceneItem::axis(
                SceneAxisRule {
                    from: (0.0, 0.0),
                    to: (0.0, 200.0)
                },
                vec![
                    SceneAxisTick {
                        position: (0.0, 0.0),
                        label: Some("0.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 20.0),
                        label: Some("10.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 40.0),
                        label: Some("20.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 60.000004),
                        label: Some("30.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 80.0),
                        label: Some("40.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 100.0),
                        label: Some("50.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 120.00001),
                        label: Some("60.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 140.0),
                        label: Some("70.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 160.0),
                        label: Some("80.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 180.0),
                        label: Some("90.00".to_string())
                    },
                    SceneAxisTick {
                        position: (0.0, 200.0),
                        label: Some("100.00".to_string())
                    }
                ],
                AxisOrientation::Left
            )])
        )
    }

    #[tokio::test]
    async fn creates_right_axis() {
        let operator = AxisOperator::new(
            Axis::new("vertical", AxisOrientation::Right),
            Scale::linear((0.0, 200.0)),
            SceneWindow::new(200, 100),
        );

        let pulse = operator
            .evaluate(Pulse::domain(ResolvedDomain::Interval(0.0, 100.0)))
            .await;

        assert_eq!(
            pulse,
            Pulse::shapes(vec![SceneItem::axis(
                SceneAxisRule {
                    from: (200.0, 0.0),
                    to: (200.0, 200.0)
                },
                vec![
                    SceneAxisTick {
                        position: (200.0, 0.0),
                        label: Some("0.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 20.0),
                        label: Some("10.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 40.0),
                        label: Some("20.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 60.000004),
                        label: Some("30.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 80.0),
                        label: Some("40.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 100.0),
                        label: Some("50.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 120.00001),
                        label: Some("60.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 140.0),
                        label: Some("70.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 160.0),
                        label: Some("80.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 180.0),
                        label: Some("90.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 200.0),
                        label: Some("100.00".to_string())
                    }
                ],
                AxisOrientation::Right
            )])
        )
    }

    #[tokio::test]
    async fn creates_axis_with_positive_min() {
        let operator = AxisOperator::new(
            Axis::new("vertical", AxisOrientation::Right),
            Scale::linear((100.0, 200.0)),
            SceneWindow::new(200, 100),
        );

        let pulse = operator
            .evaluate(Pulse::domain(ResolvedDomain::Interval(20.0, 100.0)))
            .await;

        assert_eq!(
            pulse,
            Pulse::shapes(vec![SceneItem::axis(
                SceneAxisRule {
                    from: (200.0, 100.0),
                    to: (200.0, 200.0)
                },
                vec![
                    SceneAxisTick {
                        position: (200.0, 100.0),
                        label: Some("20.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 110.0),
                        label: Some("28.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 120.0),
                        label: Some("36.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 130.0),
                        label: Some("44.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 140.0),
                        label: Some("52.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 150.0),
                        label: Some("60.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 160.0),
                        label: Some("68.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 170.0),
                        label: Some("76.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 180.0),
                        label: Some("84.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 190.0),
                        label: Some("92.00".to_string())
                    },
                    SceneAxisTick {
                        position: (200.0, 200.0),
                        label: Some("100.00".to_string())
                    }
                ],
                AxisOrientation::Right
            )])
        )
    }
}

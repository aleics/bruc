use crate::spec::scale::range::Range;
use crate::spec::scale::Scale as ScaleSpec;
use crate::spec::scale::ScaleKind as ScaleKindSpec;

const DEFAULT_TICKS_COUNT: usize = 10;
const EPSILON: f32 = 1e-10;
const EPISLON_LOG: f32 = -10.0;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleTick {
    pub(crate) value: f32,
    pub(crate) position: f32,
    pub(crate) label: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Scale {
    kind: ScaleKind,
}

impl Scale {
    pub(crate) fn from_spec(scale: &ScaleSpec) -> Self {
        match &scale.kind {
            ScaleKindSpec::Linear(linear) => {
                let Range::Literal(min, max) = linear.range;
                Scale::linear((min, max))
            }
            ScaleKindSpec::Band(band) => {
                let Range::Literal(min, max) = band.range;
                Scale::band((min, max))
            }
            ScaleKindSpec::Log(log) => {
                let Range::Literal(min, max) = log.range;
                Scale::log((min, max))
            }
        }
    }

    pub(crate) fn linear(range: (f32, f32)) -> Self {
        Scale {
            kind: ScaleKind::Linear(ScaleLinear { range }),
        }
    }

    pub(crate) fn band(range: (f32, f32)) -> Self {
        Scale {
            kind: ScaleKind::Band(ScaleBand { range }),
        }
    }

    pub(crate) fn log(range: (f32, f32)) -> Self {
        Scale {
            kind: ScaleKind::Log(ScaleLog { range }),
        }
    }

    pub(crate) fn range(&self) -> (f32, f32) {
        match &self.kind {
            ScaleKind::Linear(linear) => linear.range,
            ScaleKind::Log(log) => log.range,
            ScaleKind::Band(band) => band.range,
        }
    }

    pub(crate) fn ticks(&self, domain: (f32, f32)) -> Vec<ScaleTick> {
        match &self.kind {
            ScaleKind::Linear(linear) => linear.ticks(domain),
            ScaleKind::Log(log) => log.ticks(domain),
            ScaleKind::Band(band) => band.ticks(domain),
        }
    }

    pub(crate) fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
        match &self.kind {
            ScaleKind::Linear(linear) => linear.apply(value, domain),
            ScaleKind::Log(log) => log.apply(value, domain),
            ScaleKind::Band(band) => band.apply(value, domain),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum ScaleKind {
    Linear(ScaleLinear),
    Log(ScaleLog),
    Band(ScaleBand),
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleLinear {
    pub(crate) range: (f32, f32),
}

impl ScaleLinear {
    fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
        interpolate(normalize(value, domain), self.range)
    }

    fn ticks(&self, domain: (f32, f32)) -> Vec<ScaleTick> {
        create_tick_relative_positions(DEFAULT_TICKS_COUNT, domain)
            .into_iter()
            .map(|value| ScaleTick {
                position: self.apply(value, domain),
                value,
                label: Some(format!("{:.2}", value)),
            })
            .collect()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleLog {
    pub(crate) range: (f32, f32),
}

impl ScaleLog {
    fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
        interpolate(normalize_log10(value, domain), self.range)
    }

    fn ticks(&self, domain: (f32, f32)) -> Vec<ScaleTick> {
        let from_axis = domain.0.max(EPSILON).log10() as i32;
        let to_axis = domain.1.max(EPSILON).log10() as i32;

        let mut ticks = Vec::new();

        for exp in from_axis..=to_axis {
            let log_tick = 10f32.powf(exp as f32);

            for i in 1..10 {
                let value = log_tick * (i as f32);
                if value >= domain.0 && value <= domain.1 {
                    ticks.push(ScaleTick {
                        position: self.apply(value, domain),
                        value,
                        label: (i == 1).then(|| Self::format(value, exp)),
                    });
                }
            }
        }

        if let Some(tick) = ticks.first_mut() {
            tick.label = Some(Self::format(tick.value, from_axis))
        }

        if let Some(tick) = ticks.last_mut() {
            tick.label = Some(Self::format(tick.value, to_axis))
        }

        ticks
    }

    fn format(value: f32, exp: i32) -> String {
        match exp {
            -2..=2 => format!("{}", value),
            _ => format!("10e{:}", exp),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleBand {
    pub(crate) range: (f32, f32),
}

impl ScaleBand {
    fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
        let count = domain.1 - domain.0 + 1.0;

        let step = (self.range.1 - self.range.0) / count;
        let padding = step / 2.0;
        let range = (self.range.0 + padding, self.range.1 - padding);

        Self::calculate(value, domain, range)
    }

    fn ticks(&self, domain: (f32, f32)) -> Vec<ScaleTick> {
        let count = domain.1 - domain.0;

        let step = (self.range.1 - self.range.0) / (count + 1.0);
        let padding = step / 2.0;
        let range = (self.range.0 + padding, self.range.1 - padding);

        create_tick_relative_positions(count as usize, domain)
            .into_iter()
            .map(|value| ScaleTick {
                position: Self::calculate(value, domain, range),
                value,
                label: Some(format!("{:.2}", value)),
            })
            .collect()
    }

    fn calculate(value: f32, domain: (f32, f32), range: (f32, f32)) -> f32 {
        interpolate(normalize(value, domain), range)
    }
}

fn normalize(value: f32, (min, max): (f32, f32)) -> f32 {
    let value = value.clamp(min, max);
    (value - min) / (max - min)
}

fn normalize_by<F>(value: f32, (min, max): (f32, f32), by: F) -> f32
where
    F: Fn(f32) -> f32,
{
    normalize(by(value), (by(min), by(max)))
}

fn logn(value: f32) -> f32 {
    if value > 0.0 {
        value.log10()
    } else if value < 0.0 {
        -f32::log10(-value)
    } else {
        EPISLON_LOG
    }
}

fn normalize_log10(value: f32, domain: (f32, f32)) -> f32 {
    normalize_by(value, domain, logn)
}

fn interpolate(value: f32, (min, max): (f32, f32)) -> f32 {
    (max - min) * value + min
}

fn create_tick_relative_positions(count: usize, (from, to): (f32, f32)) -> Vec<f32> {
    let step = (to - from) / (count as f32);
    (0..count + 1).map(|i| from + step * (i as f32)).collect()
}

#[cfg(test)]
mod tests {
    use crate::scale::ScaleTick;

    use super::Scale;

    #[test]
    fn create_linear_ticks() {
        // given
        let scale = Scale::linear((0.0, 200.0));

        // when, then
        assert_eq!(
            scale.ticks((0.0, 1000.0)),
            vec![
                ScaleTick {
                    value: 0.0,
                    position: 0.0,
                    label: Some("0.00".to_string())
                },
                ScaleTick {
                    value: 100.0,
                    position: 20.0,
                    label: Some("100.00".to_string())
                },
                ScaleTick {
                    value: 200.0,
                    position: 40.0,
                    label: Some("200.00".to_string())
                },
                ScaleTick {
                    value: 300.0,
                    position: 60.000004,
                    label: Some("300.00".to_string())
                },
                ScaleTick {
                    value: 400.0,
                    position: 80.0,
                    label: Some("400.00".to_string())
                },
                ScaleTick {
                    value: 500.0,
                    position: 100.0,
                    label: Some("500.00".to_string())
                },
                ScaleTick {
                    value: 600.0,
                    position: 120.00001,
                    label: Some("600.00".to_string())
                },
                ScaleTick {
                    value: 700.0,
                    position: 140.0,
                    label: Some("700.00".to_string())
                },
                ScaleTick {
                    value: 800.0,
                    position: 160.0,
                    label: Some("800.00".to_string())
                },
                ScaleTick {
                    value: 900.0,
                    position: 180.0,
                    label: Some("900.00".to_string())
                },
                ScaleTick {
                    value: 1000.0,
                    position: 200.0,
                    label: Some("1000.00".to_string())
                }
            ]
        );
    }

    #[test]
    fn create_log_ticks() {
        // given
        let scale = Scale::log((0.0, 200.0));

        // when, then
        assert_eq!(
            scale.ticks((1.0, 1000.0)),
            vec![
                ScaleTick {
                    value: 1.0,
                    position: 0.0,
                    label: Some("1".to_string())
                },
                ScaleTick {
                    value: 2.0,
                    position: 20.068668,
                    label: None
                },
                ScaleTick {
                    value: 3.0,
                    position: 31.808084,
                    label: None
                },
                ScaleTick {
                    value: 4.0,
                    position: 40.137337,
                    label: None
                },
                ScaleTick {
                    value: 5.0,
                    position: 46.598003,
                    label: None
                },
                ScaleTick {
                    value: 6.0,
                    position: 51.876755,
                    label: None
                },
                ScaleTick {
                    value: 7.0,
                    position: 56.339867,
                    label: None
                },
                ScaleTick {
                    value: 8.0,
                    position: 60.206,
                    label: None
                },
                ScaleTick {
                    value: 9.0,
                    position: 63.61617,
                    label: None
                },
                ScaleTick {
                    value: 10.0,
                    position: 66.66667,
                    label: Some("10".to_string())
                },
                ScaleTick {
                    value: 20.0,
                    position: 86.73534,
                    label: None
                },
                ScaleTick {
                    value: 30.0,
                    position: 98.47475,
                    label: None
                },
                ScaleTick {
                    value: 40.0,
                    position: 106.804,
                    label: None
                },
                ScaleTick {
                    value: 50.0,
                    position: 113.26467,
                    label: None
                },
                ScaleTick {
                    value: 60.0,
                    position: 118.54342,
                    label: None
                },
                ScaleTick {
                    value: 70.0,
                    position: 123.00653,
                    label: None
                },
                ScaleTick {
                    value: 80.0,
                    position: 126.87267,
                    label: None
                },
                ScaleTick {
                    value: 90.0,
                    position: 130.28284,
                    label: None
                },
                ScaleTick {
                    value: 100.0,
                    position: 133.33334,
                    label: Some("100".to_string())
                },
                ScaleTick {
                    value: 200.0,
                    position: 153.402,
                    label: None
                },
                ScaleTick {
                    value: 300.0,
                    position: 165.14143,
                    label: None
                },
                ScaleTick {
                    value: 400.0,
                    position: 173.47067,
                    label: None
                },
                ScaleTick {
                    value: 500.0,
                    position: 179.93134,
                    label: None
                },
                ScaleTick {
                    value: 600.0,
                    position: 185.21008,
                    label: None
                },
                ScaleTick {
                    value: 700.0,
                    position: 189.6732,
                    label: None
                },
                ScaleTick {
                    value: 800.0,
                    position: 193.53934,
                    label: None
                },
                ScaleTick {
                    value: 900.0,
                    position: 196.9495,
                    label: None
                },
                ScaleTick {
                    value: 1000.0,
                    position: 200.0,
                    label: Some("10e3".to_string())
                }
            ]
        );
        assert_eq!(
            scale.ticks((2.5, 99.0)),
            vec![
                ScaleTick {
                    value: 3.0,
                    position: 9.911936,
                    label: Some("3".to_string())
                },
                ScaleTick {
                    value: 4.0,
                    position: 25.551811,
                    label: None
                },
                ScaleTick {
                    value: 5.0,
                    position: 37.683037,
                    label: None
                },
                ScaleTick {
                    value: 6.0,
                    position: 47.59497,
                    label: None
                },
                ScaleTick {
                    value: 7.0,
                    position: 55.975388,
                    label: None
                },
                ScaleTick {
                    value: 8.0,
                    position: 63.234837,
                    label: None
                },
                ScaleTick {
                    value: 9.0,
                    position: 69.63813,
                    label: None
                },
                ScaleTick {
                    value: 10.0,
                    position: 75.36606,
                    label: Some("10".to_string())
                },
                ScaleTick {
                    value: 20.0,
                    position: 113.0491,
                    label: None
                },
                ScaleTick {
                    value: 30.0,
                    position: 135.09225,
                    label: None
                },
                ScaleTick {
                    value: 40.0,
                    position: 150.73212,
                    label: None
                },
                ScaleTick {
                    value: 50.0,
                    position: 162.86334,
                    label: None
                },
                ScaleTick {
                    value: 60.0,
                    position: 172.7753,
                    label: None
                },
                ScaleTick {
                    value: 70.0,
                    position: 181.15572,
                    label: None
                },
                ScaleTick {
                    value: 80.0,
                    position: 188.41518,
                    label: None
                },
                ScaleTick {
                    value: 90.0,
                    position: 194.81845,
                    label: Some("90".to_string())
                }
            ]
        );
        assert_eq!(
            scale.ticks((0.001, 1.0)),
            vec![
                ScaleTick {
                    value: 0.001,
                    position: 0.0,
                    label: Some("10e-3".to_string())
                },
                ScaleTick {
                    value: 0.002,
                    position: 20.068663,
                    label: None
                },
                ScaleTick {
                    value: 0.003,
                    position: 31.80809,
                    label: None
                },
                ScaleTick {
                    value: 0.004,
                    position: 40.137337,
                    label: None
                },
                ScaleTick {
                    value: 0.0050000004,
                    position: 46.598007,
                    label: None
                },
                ScaleTick {
                    value: 0.006,
                    position: 51.876755,
                    label: None
                },
                ScaleTick {
                    value: 0.007,
                    position: 56.339867,
                    label: None
                },
                ScaleTick {
                    value: 0.008,
                    position: 60.206,
                    label: None
                },
                ScaleTick {
                    value: 0.009000001,
                    position: 63.61616,
                    label: None
                },
                ScaleTick {
                    value: 0.01,
                    position: 66.66667,
                    label: Some("0.01".to_string())
                },
                ScaleTick {
                    value: 0.02,
                    position: 86.73534,
                    label: None
                },
                ScaleTick {
                    value: 0.03,
                    position: 98.47475,
                    label: None
                },
                ScaleTick {
                    value: 0.04,
                    position: 106.804,
                    label: None
                },
                ScaleTick {
                    value: 0.049999997,
                    position: 113.26467,
                    label: None
                },
                ScaleTick {
                    value: 0.06,
                    position: 118.54342,
                    label: None
                },
                ScaleTick {
                    value: 0.07,
                    position: 123.00653,
                    label: None
                },
                ScaleTick {
                    value: 0.08,
                    position: 126.87267,
                    label: None
                },
                ScaleTick {
                    value: 0.089999996,
                    position: 130.28284,
                    label: None
                },
                ScaleTick {
                    value: 0.1,
                    position: 133.33334,
                    label: Some("0.1".to_string())
                },
                ScaleTick {
                    value: 0.2,
                    position: 153.402,
                    label: None
                },
                ScaleTick {
                    value: 0.3,
                    position: 165.14143,
                    label: None
                },
                ScaleTick {
                    value: 0.4,
                    position: 173.47067,
                    label: None
                },
                ScaleTick {
                    value: 0.5,
                    position: 179.93134,
                    label: None
                },
                ScaleTick {
                    value: 0.6,
                    position: 185.21008,
                    label: None
                },
                ScaleTick {
                    value: 0.7,
                    position: 189.6732,
                    label: None
                },
                ScaleTick {
                    value: 0.8,
                    position: 193.53934,
                    label: None
                },
                ScaleTick {
                    value: 0.90000004,
                    position: 196.9495,
                    label: None
                },
                ScaleTick {
                    value: 1.0,
                    position: 200.0,
                    label: Some("1".to_string())
                }
            ]
        );
        assert_eq!(
            scale.ticks((0.0, 1.0)),
            vec![
                ScaleTick {
                    value: 1e-10,
                    position: 0.0,
                    label: Some("10e-10".to_string())
                },
                ScaleTick {
                    value: 2e-10,
                    position: 6.020603,
                    label: None
                },
                ScaleTick {
                    value: 3e-10,
                    position: 9.542427,
                    label: None
                },
                ScaleTick {
                    value: 4e-10,
                    position: 12.041206,
                    label: None
                },
                ScaleTick {
                    value: 5e-10,
                    position: 13.979396,
                    label: None
                },
                ScaleTick {
                    value: 6e-10,
                    position: 15.56303,
                    label: None
                },
                ScaleTick {
                    value: 7e-10,
                    position: 16.90197,
                    label: None
                },
                ScaleTick {
                    value: 8e-10,
                    position: 18.06179,
                    label: None
                },
                ScaleTick {
                    value: 9e-10,
                    position: 19.084854,
                    label: None
                },
                ScaleTick {
                    value: 1e-9,
                    position: 20.0,
                    label: Some("10e-9".to_string())
                },
                ScaleTick {
                    value: 2e-9,
                    position: 26.020605,
                    label: None
                },
                ScaleTick {
                    value: 2.9999998e-9,
                    position: 29.542429,
                    label: None
                },
                ScaleTick {
                    value: 4e-9,
                    position: 32.041206,
                    label: None
                },
                ScaleTick {
                    value: 5e-9,
                    position: 33.979397,
                    label: None
                },
                ScaleTick {
                    value: 5.9999996e-9,
                    position: 35.56303,
                    label: None
                },
                ScaleTick {
                    value: 6.9999997e-9,
                    position: 36.90197,
                    label: None
                },
                ScaleTick {
                    value: 8e-9,
                    position: 38.06179,
                    label: None
                },
                ScaleTick {
                    value: 9e-9,
                    position: 39.084854,
                    label: None
                },
                ScaleTick {
                    value: 1e-8,
                    position: 40.0,
                    label: Some("10e-8".to_string())
                },
                ScaleTick {
                    value: 2e-8,
                    position: 46.020603,
                    label: None
                },
                ScaleTick {
                    value: 3e-8,
                    position: 49.542427,
                    label: None
                },
                ScaleTick {
                    value: 4e-8,
                    position: 52.0412,
                    label: None
                },
                ScaleTick {
                    value: 5e-8,
                    position: 53.979397,
                    label: None
                },
                ScaleTick {
                    value: 6e-8,
                    position: 55.56302,
                    label: None
                },
                ScaleTick {
                    value: 7e-8,
                    position: 56.901962,
                    label: None
                },
                ScaleTick {
                    value: 8e-8,
                    position: 58.061802,
                    label: None
                },
                ScaleTick {
                    value: 9e-8,
                    position: 59.084858,
                    label: None
                },
                ScaleTick {
                    value: 1e-7,
                    position: 60.000004,
                    label: Some("10e-7".to_string())
                },
                ScaleTick {
                    value: 2e-7,
                    position: 66.0206,
                    label: None
                },
                ScaleTick {
                    value: 3e-7,
                    position: 69.54243,
                    label: None
                },
                ScaleTick {
                    value: 4e-7,
                    position: 72.0412,
                    label: None
                },
                ScaleTick {
                    value: 5e-7,
                    position: 73.97939,
                    label: None
                },
                ScaleTick {
                    value: 6e-7,
                    position: 75.56302,
                    label: None
                },
                ScaleTick {
                    value: 7e-7,
                    position: 76.90196,
                    label: None
                },
                ScaleTick {
                    value: 8e-7,
                    position: 78.0618,
                    label: None
                },
                ScaleTick {
                    value: 9.0000003e-7,
                    position: 79.084854,
                    label: None
                },
                ScaleTick {
                    value: 1e-6,
                    position: 80.0,
                    label: Some("10e-6".to_string())
                },
                ScaleTick {
                    value: 2e-6,
                    position: 86.0206,
                    label: None
                },
                ScaleTick {
                    value: 3e-6,
                    position: 89.54243,
                    label: None
                },
                ScaleTick {
                    value: 4e-6,
                    position: 92.04119,
                    label: None
                },
                ScaleTick {
                    value: 5e-6,
                    position: 93.97939,
                    label: None
                },
                ScaleTick {
                    value: 6e-6,
                    position: 95.56302,
                    label: None
                },
                ScaleTick {
                    value: 7e-6,
                    position: 96.90196,
                    label: None
                },
                ScaleTick {
                    value: 8e-6,
                    position: 98.0618,
                    label: None
                },
                ScaleTick {
                    value: 9e-6,
                    position: 99.084854,
                    label: None
                },
                ScaleTick {
                    value: 1e-5,
                    position: 100.0,
                    label: Some("10e-5".to_string())
                },
                ScaleTick {
                    value: 2e-5,
                    position: 106.02061,
                    label: None
                },
                ScaleTick {
                    value: 3e-5,
                    position: 109.54243,
                    label: None
                },
                ScaleTick {
                    value: 4e-5,
                    position: 112.0412,
                    label: None
                },
                ScaleTick {
                    value: 5e-5,
                    position: 113.9794,
                    label: None
                },
                ScaleTick {
                    value: 6e-5,
                    position: 115.56303,
                    label: None
                },
                ScaleTick {
                    value: 6.9999995e-5,
                    position: 116.901955,
                    label: None
                },
                ScaleTick {
                    value: 8e-5,
                    position: 118.061806,
                    label: None
                },
                ScaleTick {
                    value: 9e-5,
                    position: 119.08486,
                    label: None
                },
                ScaleTick {
                    value: 0.0001,
                    position: 120.00001,
                    label: Some("10e-4".to_string())
                },
                ScaleTick {
                    value: 0.0002,
                    position: 126.0206,
                    label: None
                },
                ScaleTick {
                    value: 0.00029999999,
                    position: 129.54242,
                    label: None
                },
                ScaleTick {
                    value: 0.0004,
                    position: 132.0412,
                    label: None
                },
                ScaleTick {
                    value: 0.00049999997,
                    position: 133.97939,
                    label: None
                },
                ScaleTick {
                    value: 0.00059999997,
                    position: 135.56303,
                    label: None
                },
                ScaleTick {
                    value: 0.0007,
                    position: 136.90196,
                    label: None
                },
                ScaleTick {
                    value: 0.0008,
                    position: 138.0618,
                    label: None
                },
                ScaleTick {
                    value: 0.0009,
                    position: 139.08485,
                    label: None
                },
                ScaleTick {
                    value: 0.001,
                    position: 140.0,
                    label: Some("10e-3".to_string())
                },
                ScaleTick {
                    value: 0.002,
                    position: 146.0206,
                    label: None
                },
                ScaleTick {
                    value: 0.003,
                    position: 149.54242,
                    label: None
                },
                ScaleTick {
                    value: 0.004,
                    position: 152.04121,
                    label: None
                },
                ScaleTick {
                    value: 0.0050000004,
                    position: 153.9794,
                    label: None
                },
                ScaleTick {
                    value: 0.006,
                    position: 155.56303,
                    label: None
                },
                ScaleTick {
                    value: 0.007,
                    position: 156.90196,
                    label: None
                },
                ScaleTick {
                    value: 0.008,
                    position: 158.0618,
                    label: None
                },
                ScaleTick {
                    value: 0.009000001,
                    position: 159.08485,
                    label: None
                },
                ScaleTick {
                    value: 0.01,
                    position: 160.0,
                    label: Some("0.01".to_string())
                },
                ScaleTick {
                    value: 0.02,
                    position: 166.02061,
                    label: None
                },
                ScaleTick {
                    value: 0.03,
                    position: 169.54243,
                    label: None
                },
                ScaleTick {
                    value: 0.04,
                    position: 172.0412,
                    label: None
                },
                ScaleTick {
                    value: 0.049999997,
                    position: 173.9794,
                    label: None
                },
                ScaleTick {
                    value: 0.06,
                    position: 175.56302,
                    label: None
                },
                ScaleTick {
                    value: 0.07,
                    position: 176.90198,
                    label: None
                },
                ScaleTick {
                    value: 0.08,
                    position: 178.0618,
                    label: None
                },
                ScaleTick {
                    value: 0.089999996,
                    position: 179.08485,
                    label: None
                },
                ScaleTick {
                    value: 0.1,
                    position: 180.0,
                    label: Some("0.1".to_string())
                },
                ScaleTick {
                    value: 0.2,
                    position: 186.0206,
                    label: None
                },
                ScaleTick {
                    value: 0.3,
                    position: 189.54242,
                    label: None
                },
                ScaleTick {
                    value: 0.4,
                    position: 192.0412,
                    label: None
                },
                ScaleTick {
                    value: 0.5,
                    position: 193.9794,
                    label: None
                },
                ScaleTick {
                    value: 0.6,
                    position: 195.56303,
                    label: None
                },
                ScaleTick {
                    value: 0.7,
                    position: 196.90196,
                    label: None
                },
                ScaleTick {
                    value: 0.8,
                    position: 198.06178,
                    label: None
                },
                ScaleTick {
                    value: 0.90000004,
                    position: 199.08485,
                    label: None
                },
                ScaleTick {
                    value: 1.0,
                    position: 200.0,
                    label: Some("1".to_string())
                }
            ]
        );
    }

    #[test]
    fn create_band_ticks() {
        // given
        let scale = Scale::band((0.0, 200.0));

        // when, then
        assert_eq!(
            scale.ticks((0.0, 10.0)),
            vec![
                ScaleTick {
                    value: 0.0,
                    position: 9.090909,
                    label: Some("0.00".to_string())
                },
                ScaleTick {
                    value: 1.0,
                    position: 27.272728,
                    label: Some("1.00".to_string())
                },
                ScaleTick {
                    value: 2.0,
                    position: 45.454544,
                    label: Some("2.00".to_string())
                },
                ScaleTick {
                    value: 3.0,
                    position: 63.636364,
                    label: Some("3.00".to_string())
                },
                ScaleTick {
                    value: 4.0,
                    position: 81.818184,
                    label: Some("4.00".to_string())
                },
                ScaleTick {
                    value: 5.0,
                    position: 100.0,
                    label: Some("5.00".to_string())
                },
                ScaleTick {
                    value: 6.0,
                    position: 118.18182,
                    label: Some("6.00".to_string())
                },
                ScaleTick {
                    value: 7.0,
                    position: 136.36363,
                    label: Some("7.00".to_string())
                },
                ScaleTick {
                    value: 8.0,
                    position: 154.54546,
                    label: Some("8.00".to_string())
                },
                ScaleTick {
                    value: 9.0,
                    position: 172.72726,
                    label: Some("9.00".to_string())
                },
                ScaleTick {
                    value: 10.0,
                    position: 190.90909,
                    label: Some("10.00".to_string())
                }
            ]
        );
    }
}

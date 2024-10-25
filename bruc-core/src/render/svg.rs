use core::f32;

use crate::{
    scene::{
        SceneArc, SceneAxis, SceneAxisTick, SceneDimensions, SceneGroup, SceneItem, SceneLine,
        ScenePoint, SceneRect, SceneRoot, Scenegraph,
    },
    spec::axis::AxisOrientation,
};

use super::{ItemRenderer, SceneRenderer};

const SVG_TICK_SIZE: f32 = 5.0;
const SVG_AXIS_COLOR: &str = "#212121";
const SVG_AXIS_MARGIN: (f32, f32) = (35.0, 20.0);
const SVG_CANVAS_MARGIN: (f32, f32) = (10.0, 10.0);

pub struct SvgRenderer;

impl SvgRenderer {
    fn render_root(root: &SceneRoot) -> String {
        let result = root.render(&root.dimensions);

        let canvas_margin_x = result.d_width + SVG_CANVAS_MARGIN.0;
        let canvas_margin_y = result.d_height + SVG_CANVAS_MARGIN.1;

        format!(
            "<svg width=\"{width}\" height=\"{height}\"><g transform=\"translate({margin_x}, {margin_y})\">{content}</g></svg>",
            width = root.dimensions.width as f32 + canvas_margin_x.max(SVG_CANVAS_MARGIN.0 * 2.0),
            height = root.dimensions.height as f32 + canvas_margin_y.max(SVG_CANVAS_MARGIN.1 * 2.0),
            margin_x = result.margin.0.max(SVG_CANVAS_MARGIN.0),
            margin_y = result.margin.1.max(SVG_CANVAS_MARGIN.1),
            content = result.content
        )
    }
}

impl SceneRenderer for SvgRenderer {
    fn render(&self, scene: &Scenegraph) -> String {
        SvgRenderer::render_root(&scene.root)
    }
}

#[derive(Default)]
pub(crate) struct SvgRenderResult {
    content: String,
    d_width: f32,
    d_height: f32,
    margin: (f32, f32),
}

impl SvgRenderResult {
    fn merge(&mut self, other: &SvgRenderResult) {
        self.content += &other.content;
        self.d_width += other.d_width;
        self.d_height += other.d_height;
        self.margin = (
            self.margin.0 + other.margin.0,
            self.margin.1 + other.margin.1,
        );
    }
}

impl ItemRenderer for SceneRoot {
    type RenderResult = SvgRenderResult;

    fn render(&self, dimensions: &SceneDimensions) -> Self::RenderResult {
        self.items
            .iter()
            .fold(SvgRenderResult::default(), |mut acc, item| {
                acc.merge(&item.render(dimensions));
                acc
            })
    }
}

impl ItemRenderer for SceneItem {
    type RenderResult = SvgRenderResult;

    fn render(&self, dimensions: &SceneDimensions) -> Self::RenderResult {
        match self {
            SceneItem::Group(group) => group.render(dimensions),
            SceneItem::Line(line) => line.render(dimensions),
            SceneItem::Rect(rect) => rect.render(dimensions),
            SceneItem::Axis(axis) => axis.render(dimensions),
            SceneItem::Arc(arc) => arc.render(dimensions),
            SceneItem::Point(point) => point.render(dimensions),
        }
    }
}

impl ItemRenderer for SceneGroup {
    type RenderResult = SvgRenderResult;

    fn render(&self, dimensions: &SceneDimensions) -> Self::RenderResult {
        let result = self
            .items
            .iter()
            .fold(SvgRenderResult::default(), |mut acc, item| {
                acc.merge(&item.render(dimensions));
                acc
            });

        SvgRenderResult {
            content: format!("<g>{content}</g>", content = result.content),
            d_width: result.d_width,
            d_height: result.d_height,
            margin: result.margin,
        }
    }
}

impl ItemRenderer for SceneLine {
    type RenderResult = SvgRenderResult;

    fn render(&self, _: &SceneDimensions) -> Self::RenderResult {
        let path =
            self.points
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc, (index, (x, y))| {
                    let coordinates = if index == 0 {
                        format!("M{} {}", x, y)
                    } else {
                        format!(" L{} {}", x, y)
                    };
                    acc.push_str(&coordinates);
                    acc
                });

        let stroke = &self.stroke;
        let stroke_width = self.stroke_width;

        SvgRenderResult {
            content: format!("<path d=\"{path}\" fill=\"transparent\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\" stroke-linecap=\"round\" />"),
            d_width: 0.0,
            d_height: 0.0,
            margin: (0.0, 0.0)
        }
    }
}

impl ItemRenderer for SceneArc {
    type RenderResult = SvgRenderResult;

    fn render(&self, _dimensions: &SceneDimensions) -> Self::RenderResult {
        let a0 = angle_to_radians(self.start_angle);
        let a1 = angle_to_radians(self.end_angle);

        let x0 = f32::cos(a0);
        let y0 = f32::sin(a0);

        let x1 = f32::cos(a1);
        let y1 = f32::sin(a1);

        let large_arc = if (self.end_angle - self.start_angle) > 180.0 {
            1
        } else {
            0
        };

        let outer_radius = self
            .outer_radius
            .map(|outer_radius| self.inner_radius + outer_radius)
            .unwrap_or(self.radius);

        let bottom_left = (
            self.radius + self.inner_radius * x0,
            self.radius + self.inner_radius * y0,
        );

        let top_left = (
            self.radius + outer_radius * x0,
            self.radius + outer_radius * y0,
        );

        let top_right = (
            self.radius + outer_radius * x1,
            self.radius + outer_radius * y1,
        );

        let bottom_right = (
            self.radius + self.inner_radius * x1,
            self.radius + self.inner_radius * y1,
        );

        let path = format!(
            "M {} {} L {}, {} A {} {} 0 {} 1 {} {} L {} {} A {} {} 0 {} 0 {} {}",
            bottom_left.0,
            bottom_left.1,
            top_left.0,
            top_left.1,
            outer_radius,
            outer_radius,
            large_arc,
            top_right.0,
            top_right.1,
            bottom_right.0,
            bottom_right.1,
            self.inner_radius,
            self.inner_radius,
            large_arc,
            bottom_left.0,
            bottom_left.1
        );

        let fill = &self.fill;
        SvgRenderResult {
            content: format!("<path d=\"{path}\" fill=\"{fill}\"/>"),
            d_width: 0.0,
            d_height: 0.0,
            margin: (0.0, 0.0),
        }
    }
}

fn angle_to_radians(angle: f32) -> f32 {
    (angle - 90.0) * f32::consts::PI / 180.0
}

impl ItemRenderer for SceneAxis {
    type RenderResult = SvgRenderResult;

    fn render(&self, dimensions: &SceneDimensions) -> Self::RenderResult {
        match self.orientation {
            AxisOrientation::Top => render_top_axis(self, dimensions),
            AxisOrientation::Bottom => render_bottom_axis(self, dimensions),
            AxisOrientation::Left => render_left_axis(self, dimensions),
            AxisOrientation::Right => render_right_axis(self, dimensions),
        }
    }
}

fn render_top_axis(axis: &SceneAxis, dimensions: &SceneDimensions) -> SvgRenderResult {
    let tick_size = (0.0, -SVG_TICK_SIZE);
    let tick_text_margin = (0.0, -0.5);

    let ticks = render_axis_ticks(&axis.ticks, tick_size, tick_text_margin, dimensions);
    let ruler = render_axis_ruler(axis, dimensions);

    SvgRenderResult {
        content: format!("<g>{ticks}{ruler}</g>"),
        d_width: 0.0,
        d_height: SVG_AXIS_MARGIN.1,
        margin: (0.0, SVG_AXIS_MARGIN.1),
    }
}

fn render_bottom_axis(axis: &SceneAxis, dimensions: &SceneDimensions) -> SvgRenderResult {
    let tick_size = (0.0, SVG_TICK_SIZE);
    let tick_text_margin = (0.0, 1.0);

    let ticks = render_axis_ticks(&axis.ticks, tick_size, tick_text_margin, dimensions);
    let ruler = render_axis_ruler(axis, dimensions);

    SvgRenderResult {
        content: format!("<g>{ticks}{ruler}</g>"),
        d_width: 0.0,
        d_height: SVG_AXIS_MARGIN.1,
        margin: (0.0, SVG_CANVAS_MARGIN.1),
    }
}

fn render_left_axis(axis: &SceneAxis, dimensions: &SceneDimensions) -> SvgRenderResult {
    let tick_size = (-SVG_TICK_SIZE, 0.0);
    let tick_text_margin = (-0.3, 0.3);

    let ticks = render_axis_ticks(&axis.ticks, tick_size, tick_text_margin, dimensions);
    let ruler = render_axis_ruler(axis, dimensions);

    SvgRenderResult {
        content: format!("<g>{ticks}{ruler}</g>"),
        d_width: SVG_AXIS_MARGIN.0,
        d_height: 0.0,
        margin: (SVG_AXIS_MARGIN.0, 0.0),
    }
}

fn render_right_axis(axis: &SceneAxis, dimensions: &SceneDimensions) -> SvgRenderResult {
    let tick_size = (SVG_TICK_SIZE, 0.0);
    let tick_text_margin = (0.3, 0.3);

    let ticks = render_axis_ticks(&axis.ticks, tick_size, tick_text_margin, dimensions);
    let ruler = render_axis_ruler(axis, dimensions);

    SvgRenderResult {
        content: format!("<g>{ticks}{ruler}</g>"),
        d_width: SVG_AXIS_MARGIN.0,
        d_height: 0.0,
        margin: (SVG_CANVAS_MARGIN.0, 0.0),
    }
}

fn render_axis_ticks(
    ticks: &[SceneAxisTick],
    tick_size: (f32, f32),
    tick_text_margin: (f32, f32),
    dimensions: &SceneDimensions,
) -> String {
    ticks.iter().fold(String::new(), |mut acc, tick| {
    let x1 = tick.position.0;
    let x2 = tick.position.0 + tick_size.0;
    let y1 = dimensions.height as f32 - tick.position.1;
    let y2 = dimensions.height as f32 - tick.position.1 + tick_size.1;

    let tick_line = format!(
      "<line x1=\"{x1}\" x2=\"{x2}\" y1=\"{y1}\" y2=\"{y2}\" stroke-width=\"1\" opacity=\"1\" stroke=\"{SVG_AXIS_COLOR}\" stroke-linecap=\"square\" />",
    );

    let tick_text = if let Some(label) = tick.label.as_ref() {
      format!(
          "<text transform=\"translate({x2}, {y2})\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"{label_x}em\" dy=\"{label_y}em\">{label}</tspan></text>",
          label = label,
          label_x = tick_text_margin.0 * label.len() as f32,
          label_y = tick_text_margin.1
        )
    } else {
        String::new()
    };

    acc.push_str(&format!("<g>{tick_line}{tick_text}</g>"));
    acc
  })
}

fn render_axis_ruler(axis: &SceneAxis, dimensions: &SceneDimensions) -> String {
    format!(
        "<line x1=\"{x1}\" x2=\"{x2}\" y1=\"{y1}\" y2=\"{y2}\" stroke-width=\"1\" opacity=\"1\" stroke=\"{SVG_AXIS_COLOR}\" stroke-linecap=\"square\" />",
      x1 = axis.rule.from.0,
      x2 = axis.rule.to.0,
      y1 = dimensions.height as f32 - axis.rule.from.1,
      y2 = dimensions.height as f32 - axis.rule.to.1
    )
}

impl ItemRenderer for SceneRect {
    type RenderResult = SvgRenderResult;

    fn render(&self, _dimensions: &SceneDimensions) -> Self::RenderResult {
        let content = format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"{fill}\" />",
            x = self.x,
            y = self.y,
            width = self.width,
            height = self.height,
            fill = self.fill
        );

        SvgRenderResult {
            content,
            d_width: 0.0,
            d_height: 0.0,
            margin: (0.0, 0.0),
        }
    }
}

impl ItemRenderer for ScenePoint {
    type RenderResult = SvgRenderResult;

    fn render(&self, _dimensions: &SceneDimensions) -> Self::RenderResult {
        let content = format!(
            "<circle cx=\"{x}\" cy=\"{y}\" r=\"{size}\" fill=\"{color}\" />",
            x = self.x,
            y = self.y,
            size = self.size,
            color = self.color
        );

        SvgRenderResult {
            content,
            d_width: 0.0,
            d_height: 0.0,
            margin: (0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::svg::SvgRenderer;
    use crate::render::SceneRenderer;
    use crate::scene::{
        SceneAxisRule, SceneAxisTick, SceneDimensions, SceneItem, SceneRoot, Scenegraph,
    };
    use crate::spec::axis::AxisOrientation;

    #[test]
    fn render_svg_line() {
        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![SceneItem::line(
                vec![(0.0, 10.0), (1.0, 20.0)],
                "black".to_string(),
                1.0,
            )],
            SceneDimensions {
                width: 500,
                height: 200,
            },
        ));

        let renderer = SvgRenderer;
        let result = renderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"520\" height=\"220\"><g transform=\"translate(10, 10)\"><path d=\"M0 10 L1 20\" fill=\"transparent\" stroke=\"black\" stroke-width=\"1\" stroke-linecap=\"round\" /></g></svg>"
        )
    }

    #[test]
    fn render_svg_axis_top() {
        let rule = SceneAxisRule {
            from: (0.0, 0.0),
            to: (40.0, 0.0),
        };
        let ticks = vec![
            SceneAxisTick {
                position: (0.0, 0.0),
                label: Some("0.00".to_string()),
            },
            SceneAxisTick {
                position: (20.0, 0.0),
                label: Some("10.00".to_string()),
            },
            SceneAxisTick {
                position: (40.0, 0.0),
                label: Some("20.00".to_string()),
            },
        ];

        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![SceneItem::axis(rule, ticks, AxisOrientation::Top)],
            SceneDimensions {
                width: 500,
                height: 200,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"520\" height=\"230\"><g transform=\"translate(10, 20)\"><g><g><line x1=\"0\" x2=\"0\" y1=\"200\" y2=\"195\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(0, 195)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"-0.5em\">0.00</tspan></text></g><g><line x1=\"20\" x2=\"20\" y1=\"200\" y2=\"195\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(20, 195)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"-0.5em\">10.00</tspan></text></g><g><line x1=\"40\" x2=\"40\" y1=\"200\" y2=\"195\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(40, 195)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"-0.5em\">20.00</tspan></text></g><line x1=\"0\" x2=\"40\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /></g></g></svg>"
        )
    }

    #[test]
    fn render_svg_axis_bottom() {
        let rule = SceneAxisRule {
            from: (0.0, 0.0),
            to: (40.0, 0.0),
        };
        let ticks = vec![
            SceneAxisTick {
                position: (0.0, 0.0),
                label: Some("0.00".to_string()),
            },
            SceneAxisTick {
                position: (20.0, 0.0),
                label: Some("10.00".to_string()),
            },
            SceneAxisTick {
                position: (40.0, 0.0),
                label: Some("20.00".to_string()),
            },
        ];

        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![SceneItem::axis(rule, ticks, AxisOrientation::Bottom)],
            SceneDimensions {
                width: 500,
                height: 200,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"520\" height=\"230\"><g transform=\"translate(10, 10)\"><g><g><line x1=\"0\" x2=\"0\" y1=\"200\" y2=\"205\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(0, 205)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"1em\">0.00</tspan></text></g><g><line x1=\"20\" x2=\"20\" y1=\"200\" y2=\"205\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(20, 205)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"1em\">10.00</tspan></text></g><g><line x1=\"40\" x2=\"40\" y1=\"200\" y2=\"205\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(40, 205)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"0em\" dy=\"1em\">20.00</tspan></text></g><line x1=\"0\" x2=\"40\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /></g></g></svg>"
        )
    }

    #[test]
    fn render_svg_axis_left() {
        let rule = SceneAxisRule {
            from: (0.0, 0.0),
            to: (40.0, 0.0),
        };
        let ticks = vec![
            SceneAxisTick {
                position: (0.0, 0.0),
                label: Some("0.00".to_string()),
            },
            SceneAxisTick {
                position: (20.0, 0.0),
                label: Some("10.00".to_string()),
            },
            SceneAxisTick {
                position: (40.0, 0.0),
                label: Some("20.00".to_string()),
            },
        ];

        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![SceneItem::axis(rule, ticks, AxisOrientation::Left)],
            SceneDimensions {
                width: 500,
                height: 200,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"545\" height=\"220\"><g transform=\"translate(35, 10)\"><g><g><line x1=\"0\" x2=\"-5\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(-5, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"-1.2em\" dy=\"0.3em\">0.00</tspan></text></g><g><line x1=\"20\" x2=\"15\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(15, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"-1.5em\" dy=\"0.3em\">10.00</tspan></text></g><g><line x1=\"40\" x2=\"35\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(35, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"-1.5em\" dy=\"0.3em\">20.00</tspan></text></g><line x1=\"0\" x2=\"40\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /></g></g></svg>"
        )
    }

    #[test]
    fn render_svg_axis_right() {
        let rule = SceneAxisRule {
            from: (0.0, 0.0),
            to: (40.0, 0.0),
        };
        let ticks = vec![
            SceneAxisTick {
                position: (0.0, 0.0),
                label: Some("0.00".to_string()),
            },
            SceneAxisTick {
                position: (20.0, 0.0),
                label: Some("10.00".to_string()),
            },
            SceneAxisTick {
                position: (40.0, 0.0),
                label: Some("20.00".to_string()),
            },
        ];

        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![SceneItem::axis(rule, ticks, AxisOrientation::Right)],
            SceneDimensions {
                width: 500,
                height: 200,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"545\" height=\"220\"><g transform=\"translate(10, 10)\"><g><g><line x1=\"0\" x2=\"5\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(5, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"1.2em\" dy=\"0.3em\">0.00</tspan></text></g><g><line x1=\"20\" x2=\"25\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(25, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"1.5em\" dy=\"0.3em\">10.00</tspan></text></g><g><line x1=\"40\" x2=\"45\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /><text transform=\"translate(45, 200)\" dominant-baseline=\"middle\" text-anchor=\"middle\" font-size=\"10\" font-family=\"sans-serif\"><tspan dx=\"1.5em\" dy=\"0.3em\">20.00</tspan></text></g><line x1=\"0\" x2=\"40\" y1=\"200\" y2=\"200\" stroke-width=\"1\" opacity=\"1\" stroke=\"#212121\" stroke-linecap=\"square\" /></g></g></svg>"
        )
    }

    #[test]
    fn render_svg_arcs() {
        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![
                SceneItem::arc(0.0, 60.0, 250.0, 0.0, None, "blue".to_string()),
                SceneItem::arc(60.0, 120.0, 250.0, 0.0, None, "red".to_string()),
                SceneItem::arc(120.0, 180.0, 250.0, 0.0, None, "yellow".to_string()),
                SceneItem::arc(180.0, 240.0, 250.0, 0.0, None, "green".to_string()),
                SceneItem::arc(240.0, 300.0, 250.0, 0.0, None, "pink".to_string()),
                SceneItem::arc(300.0, 360.0, 250.0, 0.0, None, "black".to_string()),
            ],
            SceneDimensions {
                width: 500,
                height: 500,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"520\" height=\"520\"><g transform=\"translate(10, 10)\"><path d=\"M 250 250 L 249.99998, 0 A 250 250 0 0 1 466.50635 125 L 250 250 A 0 0 0 0 0 250 250\" fill=\"blue\"/><path d=\"M 250 250 L 466.50635, 125 A 250 250 0 0 1 466.50635 375 L 250 250 A 0 0 0 0 0 250 250\" fill=\"red\"/><path d=\"M 250 250 L 466.50635, 375 A 250 250 0 0 1 249.99998 500 L 250 250 A 0 0 0 0 0 250 250\" fill=\"yellow\"/><path d=\"M 250 250 L 249.99998, 500 A 250 250 0 0 1 33.49362 374.99994 L 250 250 A 0 0 0 0 0 250 250\" fill=\"green\"/><path d=\"M 250 250 L 33.49362, 374.99994 A 250 250 0 0 1 33.493683 124.999954 L 250 250 A 0 0 0 0 0 250 250\" fill=\"pink\"/><path d=\"M 250 250 L 33.493683, 124.999954 A 250 250 0 0 1 250 0 L 250 250 A 0 0 0 0 0 250 250\" fill=\"black\"/></g></svg>"
        )
    }

    #[test]
    fn render_svg_points() {
        let scenegraph = Scenegraph::new(SceneRoot::new(
            vec![
                SceneItem::point(0.0, 5.0, 1.0, "red".to_string()),
                SceneItem::point(1.0, 10.0, 2.0, "blue".to_string()),
                SceneItem::point(2.0, 2.0, 5.0, "green".to_string()),
                SceneItem::point(3.0, 8.0, 3.0, "orange".to_string()),
                SceneItem::point(4.0, 4.0, 1.0, "yellow".to_string()),
            ],
            SceneDimensions {
                width: 500,
                height: 500,
            },
        ));

        let result = SvgRenderer.render(&scenegraph);

        assert_eq!(
            result,
            "<svg width=\"520\" height=\"520\"><g transform=\"translate(10, 10)\"><circle cx=\"0\" cy=\"5\" r=\"1\" fill=\"red\" /><circle cx=\"1\" cy=\"10\" r=\"2\" fill=\"blue\" /><circle cx=\"2\" cy=\"2\" r=\"5\" fill=\"green\" /><circle cx=\"3\" cy=\"8\" r=\"3\" fill=\"orange\" /><circle cx=\"4\" cy=\"4\" r=\"1\" fill=\"yellow\" /></g></svg>"
        )
    }
}

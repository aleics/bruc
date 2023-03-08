use crate::scene::{SceneGroup, SceneItem, SceneLine, SceneRoot, Scenegraph};

pub trait Renderer {
  fn render(&self, scene: &Scenegraph) -> String;
}

pub struct DebugRenderer;

impl Renderer for DebugRenderer {
  fn render(&self, scene: &Scenegraph) -> String {
    format!("{scene:?}")
  }
}

pub struct SvgRenderer;

impl SvgRenderer {
  fn render_root(root: &SceneRoot) -> String {
    let width = root.width;
    let height = root.height;
    let content = root.items.iter().fold(String::new(), |acc, item| {
      acc + &SvgRenderer::render_item(item)
    });
    format!("<svg width=\"{width}\" height=\"{height}\">{content}</svg>")
  }

  fn render_group(group: &SceneGroup) -> String {
    let content = group.items.iter().fold(String::new(), |acc, item| {
      acc + &SvgRenderer::render_item(item)
    });
    format!("<g>{content}</g>")
  }

  fn render_line(line: &SceneLine) -> String {
    let x1 = line.begin.0;
    let y1 = line.begin.1;
    let x2 = line.end.0;
    let y2 = line.end.1;
    let stroke = &line.stroke;
    let stroke_width = line.stroke_width;

    format!("<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"></line>")
  }

  fn render_item(item: &SceneItem) -> String {
    match item {
      SceneItem::Group(group) => SvgRenderer::render_group(group),
      SceneItem::Line(line) => SvgRenderer::render_line(line),
    }
  }
}

impl Renderer for SvgRenderer {
  fn render(&self, scene: &Scenegraph) -> String {
    SvgRenderer::render_root(&scene.root)
  }
}

#[cfg(test)]
mod tests {
  use crate::render::{Renderer, SvgRenderer};
  use crate::scene::{SceneItem, SceneRoot, Scenegraph};

  #[test]
  fn render_svg_line() {
    let scenegraph = Scenegraph::new(SceneRoot::new(
      vec![SceneItem::line(
        (0.0, 10.0),
        (1.0, 20.0),
        "black".to_string(),
        1.0,
      )],
      500,
      200,
    ));

    let renderer = SvgRenderer;
    let result = renderer.render(&scenegraph);

    assert_eq!(
      result,
      "<svg width=\"500\" height=\"200\"><line x1=\"0\" y1=\"10\" x2=\"1\" y2=\"20\" stroke=\"black\" stroke-width=\"1\"></line></svg>"
    )
  }
}

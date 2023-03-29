use bruc_core::render::SvgRenderer;
use bruc_core::View;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Bruc {
  view: View,
}

#[wasm_bindgen]
impl Bruc {
  #[wasm_bindgen]
  pub async fn build(specification: String) -> Bruc {
    let specification = serde_json::from_str(specification.as_str()).unwrap();
    let view = View::build(specification).await;

    Bruc { view }
  }

  #[wasm_bindgen(js_name = renderAsSvg)]
  pub fn render_as_svg(self, selector: &str) {
    let window = web_sys::window().expect("No global window");
    let document = window.document().expect("No document on window");

    let element = document
      .query_selector(selector)
      .expect("Element with selector {selector} could not be queried.");

    let element = element.expect("Element not present in document.");
    let content = self.view.render(SvgRenderer);

    element.set_inner_html(&content);
  }
}

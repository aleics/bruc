use async_std::stream::StreamExt;
use bruc_core::render::svg::SvgRenderer;
use bruc_core::View;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Bruc {
  view: View,
}

#[wasm_bindgen]
impl Bruc {
  #[wasm_bindgen]
  pub fn build(specification: String) -> Self {
    let specification = serde_json::from_str(specification.as_str()).unwrap();
    let view = View::build(specification);

    Bruc { view }
  }

  #[wasm_bindgen(js_name = setData)]
  pub async fn set_data(&mut self, name: String, values: Vec<JsValue>) {
    let values = values
      .into_iter()
      .map(|value| serde_wasm_bindgen::from_value(value).unwrap())
      .collect();

    self.view.set_data(&name, values).await;
  }

  #[wasm_bindgen(js_name = renderAsSvg)]
  pub async fn render_as_svg(&mut self, selector: &str) {
    let window = web_sys::window().expect("No global window");
    let document = window.document().expect("No document on window");

    let element = document
      .query_selector(selector)
      .expect("Element with selector {selector} could not be queried.");

    let element = element.expect("Element not present in document.");

    let mut render_result = self.view.render(SvgRenderer).await;

    wasm_bindgen_futures::spawn_local(async move {
      while let Some(content) = render_result.next().await {
        element.set_inner_html(&content);
      }
    });
  }
}

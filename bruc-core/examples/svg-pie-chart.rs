use async_std::stream::StreamExt;
use bruc_core::render::svg::SvgRenderer;
use bruc_core::View;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let specification = serde_json::from_str(
    r#"{
      "dimensions": {
        "width": 500,
        "height": 500
      },
      "data": [
        {
          "name": "primary",
          "values": [
            { "x": 0, "y": 10 },
            { "x": 1, "y": 50 },
            { "x": 2, "y": 15 },
            { "x": 3, "y": 30 }
          ]
        }
      ],
      "visual": {
        "shapes": [
          {
            "from": "primary",
            "type": "pie",
            "properties": {
              "value": { "field": "y" }
            }
          }
        ]
      }
    }"#,
  )
  .unwrap();

  let mut view = View::build(specification);
  let mut render_result = view.render(SvgRenderer).await;

  let svg = render_result.next().await.unwrap();

  println!("{}", svg);
}

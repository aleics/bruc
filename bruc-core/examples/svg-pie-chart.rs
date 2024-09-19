use async_std::stream::StreamExt;
use bruc_core::render::svg::SvgRenderer;
use bruc_core::View;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let specification = serde_json::from_str(
    r#"{
      "dimensions": {
        "width": 300,
        "height": 300
      },
      "data": [
        {
          "name": "primary",
          "values": [
            { "x": 0, "y": 10, "z": 30 },
            { "x": 1, "y": 50, "z": 10 },
            { "x": 2, "y": 15, "z": 18 },
            { "x": 3, "y": 30, "z": 20 }
          ],
          "transform": [
            { "type": "map", "fn": "y * 10", "output": "value" }
          ]
        }
      ],
      "scales": [
        {
          "type": "linear",
          "name": "outer",
          "domain": { "data": "primary", "field": "z" },
          "range": [60, 110]
        }
      ],
      "visual": {
        "shapes": [
          {
            "from": "primary",
            "type": "pie",
            "properties": {
              "value": { "field": "value" },
              "innerRadius": 20,
              "outerRadius": { "field": "z", "scale": "outer" }
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

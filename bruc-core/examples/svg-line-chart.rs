use async_std::stream::StreamExt;
use bruc_core::render::svg::SvgRenderer;
use bruc_core::View;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let specification = serde_json::from_str(
    r#"{
      "dimensions": {
        "width": 500,
        "height": 200
      },
      "data": [
        {
          "name": "primary",
          "values": [
            { "x": 0, "y": 0 },
            { "x": 1, "y": 50 },
            { "x": 2, "y": 15 },
            { "x": 3, "y": 30 }
          ]
        }
      ],
      "scales": [
        {
          "type": "linear",
          "name": "horizontal",
          "domain": [0, 5],
          "range": [0, 500]
        },
        {
          "type": "linear",
          "name": "vertical",
          "domain": [0, 50],
          "range": [0, 200]
        }
      ],
      "visual": {
        "shapes": [
          {
            "from": "primary",
            "type": "line",
            "properties": {
              "x": { "field": "x", "scale": "horizontal" },
              "y": { "field": "y", "scale": "vertical" },
              "stroke": "red",
              "strokeWidth": 2
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

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
            { "x": 0, "y": 0, "color": "red" },
            { "x": 1, "y": 50, "color": "blue" },
            { "x": 2, "y": 15, "color": "green" },
            { "x": 3, "y": 30, "color": "orange" }
          ]
        }
      ],
      "scales": [
        {
          "type": "linear",
          "name": "horizontal",
          "domain": { "data": "primary", "field": "x" },
          "range": [0, 500]
        },
        {
          "type": "linear",
          "name": "vertical",
          "domain": { "data": "primary", "field": "y" },
          "range": [0, 200]
        }
      ],
      "visual": {
        "axes": [
          {
            "orientation": "top",
            "scale": "horizontal"
          },
          {
            "orientation": "bottom",
            "scale": "horizontal"
          },
          {
            "orientation": "left",
            "scale": "vertical"
          },
          {
            "orientation": "right",
            "scale": "vertical"
          }
        ],
        "shapes": [
          {
            "from": "primary",
            "type": "point",
            "properties": {
              "x": { "field": "x", "scale": "horizontal" },
              "y": { "field": "y", "scale": "vertical" },
              "color": { "field": "color" }
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

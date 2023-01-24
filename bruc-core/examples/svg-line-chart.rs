use bruc_core::render::SvgRenderer;
use bruc_core::View;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let specification = serde_json::from_str(
    r#"{
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
          "domain": [0, 10],
          "range": [0, 100]
        },
        {
          "type": "linear",
          "name": "vertical",
          "domain": [0, 100],
          "range": [0, 100]
        }
      ],
      "marks": [
        {
          "from": "primary",
          "type": "line",
          "on": {
            "update": {
              "x": { "field": "x", "scale": "horizontal" },
              "y": { "field": "y", "scale": "vertical" }
            }
          }
        }
      ]
    }"#,
  )
  .unwrap();

  let view = View::build(specification).await;
  let svg = view.render(SvgRenderer);

  println!("{}", svg);
}

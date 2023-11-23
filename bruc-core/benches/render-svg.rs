#![feature(test)]
extern crate test;

use bruc_core::{render::svg::SvgRenderer, spec::Specification, View};
use futures::StreamExt;
use rand::{thread_rng, Rng};
use test::Bencher;

fn specification(values: &str) -> String {
  format!(
    r#"
      {{  
        "dimensions": {{
          "width": 1500,
          "height": 300
        }},
        "data": [
          {{
            "name": "primary",
            "values": {values}
          }}
        ],
        "scales": [
          {{
            "type": "linear",
            "name": "horizontal",
            "domain": [0, 200],
            "range": [0, 1500]
          }},
          {{
            "type": "linear",
            "name": "vertical",
            "domain": [0, 50],
            "range": [0, 300]
          }}
        ],
        "visual": {{
          "axes": [
            {{
              "orientation": "top",
              "scale": "horizontal"
            }},
            {{
              "orientation": "bottom",
              "scale": "horizontal"
            }},
            {{
              "orientation": "left",
              "scale": "vertical"
            }},
            {{
              "orientation": "right",
              "scale": "vertical"
            }}
          ],
          "shapes": [
            {{
              "from": "primary",
              "type": "line",
              "properties": {{
                "x": {{ "field": "x", "scale": "horizontal" }},
                "y": {{ "field": "y", "scale": "vertical" }},
                "stroke": "red",
                "strokeWidth": 2
              }}
            }}
          ]
        }}
      }}
    "#
  )
}

#[bench]
fn bench_render_svg(b: &mut Bencher) {
  let mut view = create_view(1000);
  b.iter(move || {
    futures::executor::block_on(async {
      let mut result = view.render(SvgRenderer).await;
      result.next().await
    })
  })
}

fn create_view(data_amount: usize) -> View {
  let specification_json = specification(&create_data_json(data_amount));
  let spec: Specification = serde_json::from_str(&specification_json).unwrap();

  View::build(spec)
}

fn create_data_json(amount: usize) -> String {
  let mut data = String::new();
  let mut rng = thread_rng();

  for i in 0..(amount - 1) {
    let x = i as f32;
    let y: f32 = rng.gen_range(0.0..50.0);
    data.push_str(&format!(r#"{{ "x": {x}, "y": {y} }},"#));
  }
  let x = amount;
  let y: f32 = rng.gen_range(0.0..50.0);
  data.push_str(&format!(r#"{{ "x": {x}, "y": {y} }}"#));

  format!("[{data}]")
}

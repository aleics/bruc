import { Bruc } from "bruc";

const spec = `{
  "dimensions": {
    "width": 1500,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": []
    }
  ],
  "scales": [
    {
      "type": "linear",
      "name": "horizontal",
      "domain": [0, 200],
      "range": [0, 1500]
    },
    {
      "type": "linear",
      "name": "vertical",
      "domain": [0, 50],
      "range": [0, 300]
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
        "type": "line",
        "properties": {
          "x": { "field": "x", "scale": "horizontal" },
          "y": { "field": "y", "scale": "vertical" },
          "stroke": "red",
          "strokeWidth": 2
        }
      },
      {
        "from": "primary",
        "type": "line",
        "properties": {
          "x": { "field": "k", "scale": "horizontal" },
          "y": { "field": "q", "scale": "vertical" },
          "stroke": "blue",
          "strokeWidth": 1.5
        }
      }
    ]
  }
}`;

const bruc = Bruc.build(spec);

await bruc.renderAsSvg("#first");
await bruc.renderAsSvg("#second");

while(true) {
  const data = randomData();
  await bruc.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 0; i <= 200; i++) {
    values.push({ x: i, y: randomValue(50), k: i, q: randomValue(50) });
  }
  return values;
}

function randomValue(max) {
  return Math.floor(Math.random() * max);
}

function delay(time) {
  return new Promise(resolve => setTimeout(resolve, time));
}

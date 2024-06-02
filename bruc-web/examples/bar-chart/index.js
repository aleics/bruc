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
      "type": "band",
      "name": "horizontal",
      "domain": { "data": "primary", "field": "x" },
      "range": [0, 1500]
    },
    {
      "type": "linear",
      "name": "vertical",
      "domain": { "data": "primary", "field": "y" },
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
        "type": "bar",
        "properties": {
          "x": { "field": "x", "scale": "horizontal" },
          "width": 30.0,
          "height": { "field": "y", "scale": "vertical" },
          "fill": "blue"
        }
      },
      {
        "from": "primary",
        "type": "bar",
        "properties": {
          "x": { "field": "k", "scale": "horizontal" },
          "width": 30.0,
          "height": { "field": "q", "scale": "vertical" },
          "fill": "red"
        }
      }
    ]
  }
}`;

const bruc = Bruc.build(spec);

await bruc.renderAsSvg("#first");

while(true) {
  const data = randomData();
  await bruc.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 0; i <= 25; i++) {
    const y = randomValue(50);
    values.push({ x: i, y, k: i, q: Math.max(y - randomValue(20), 0) });
  }
  return values;
}

function randomValue(max) {
  return Math.floor(Math.random() * max);
}

function delay(time) {
  return new Promise(resolve => setTimeout(resolve, time));
}


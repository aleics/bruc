import { Bruc } from "bruc";

const spec_horizontal = `{
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
      "domain": { "data": "primary", "field": "k" },
      "range": [0, 1500]
    },
    {
      "type": "linear",
      "name": "vertical",
      "domain": { "data": "primary", "field": "q" },
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
          "x": { "field": "k", "scale": "horizontal" },
          "height": { "field": "q", "scale": "vertical" },
          "fill": "blue"
        }
      }
    ]
  }
}`;

const spec_vertical = `{
  "dimensions": {
    "width": 800,
    "height": 800
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
      "domain": { "data": "primary", "field": "q" },
      "range": [0, 800]
    },
    {
      "type": "band",
      "name": "vertical",
      "domain": { "data": "primary", "field": "k" },
      "range": [0, 800]
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
          "y": { "field": "k", "scale": "vertical" },
          "width": { "field": "q", "scale": "horizontal" },
          "fill": "red"
        }
      }
    ]
  }
}`;

const bruc_horizontal = Bruc.build(spec_horizontal);
await bruc_horizontal.renderAsSvg("#first");

const bruc_vertical = Bruc.build(spec_vertical);
await bruc_vertical.renderAsSvg("#second");

while(true) {
  const data = randomData();

  await bruc_horizontal.setData("primary", data);
  await bruc_vertical.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 0; i < 30; i++) {
    const y = randomValue(50);
    values.push({ k: i, q: y });
  }
  return values;
}

function randomValue(max) {
  return Math.floor(Math.random() * max);
}

function delay(time) {
  return new Promise(resolve => setTimeout(resolve, time));
}


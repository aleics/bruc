import { Bruc } from "bruc";

const pie_spec = `{
  "dimensions": {
    "width": 300,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": [],
      "transform": [
        { "type": "map", "fn": "y * 10", "output": "value" }
      ]
    }
  ],
  "visual": {
    "shapes": [
      {
        "from": "primary",
        "type": "pie",
        "properties": {
          "value": { "field": "value" },
          "padAngle": 0.02
        }
      }
    ]
  }
}`;

const donut_spec = `{
  "dimensions": {
    "width": 300,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": [],
      "transform": [
        { "type": "map", "fn": "y * 10", "output": "value" }
      ]
    }
  ],
  "visual": {
    "shapes": [
      {
        "from": "primary",
        "type": "pie",
        "properties": {
          "value": { "field": "value" },
          "padAngle": 0.02,
          "innerRadius": 120
        }
      }
    ]
  }
}`;

const radial_spec = `{
  "dimensions": {
    "width": 300,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": [],
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
      "range": [40, 160]
    }
  ],
  "visual": {
    "shapes": [
      {
        "from": "primary",
        "type": "pie",
        "properties": {
          "value": { "field": "value" },
          "outerRadius": { "field": "z", "scale": "outer" }
        }
      }
    ]
  }
}`;

const radial_donut_spec = `{
  "dimensions": {
    "width": 300,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": [],
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
      "range": [40, 110]
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
}`;

const pie = Bruc.build(pie_spec);
await pie.renderAsSvg("#pie");

const donut = Bruc.build(donut_spec);
await donut.renderAsSvg("#donut");

const radial = Bruc.build(radial_spec);
await radial.renderAsSvg("#radial");

const radial_donut = Bruc.build(radial_donut_spec);
await radial_donut.renderAsSvg("#radial-donut");

while (true) {
  const data = randomData();
  await pie.setData("primary", data);
  await donut.setData("primary", data);
  await radial.setData("primary", data);
  await radial_donut.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 0; i <= randomValue(0, 40); i++) {
    values.push({
      x: i,
      y: randomValue(0, 50),
      z: randomValue(0, 50),
    });
  }
  return values;
}

function randomValue(min, max) {
  return Math.random() * (max - min) + min;
}

function delay(time) {
  return new Promise((resolve) => setTimeout(resolve, time));
}

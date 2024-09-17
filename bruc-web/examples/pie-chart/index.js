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

const pie = Bruc.build(pie_spec);
await pie.renderAsSvg("#pie");

const donut = Bruc.build(donut_spec);
await donut.renderAsSvg("#donut");

while (true) {
  const data = randomData();
  await pie.setData("primary", data);
  await donut.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 0; i <= randomValue(40); i++) {
    values.push({ x: i, y: randomValue(50) });
  }
  return values;
}

function randomValue(max) {
  return Math.floor(Math.random() * max);
}

function delay(time) {
  return new Promise((resolve) => setTimeout(resolve, time));
}

import { Bruc } from "bruc";

const spec = `{
  "dimensions": {
    "width": 500,
    "height": 500
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

const bruc = Bruc.build(spec);

await bruc.renderAsSvg("#first");

while (true) {
  const data = randomData();
  await bruc.setData("primary", data);

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

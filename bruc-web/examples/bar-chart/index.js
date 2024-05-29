import { Bruc } from "bruc";

const spec = `{
  "dimensions": {
    "width": 1500,
    "height": 300
  },
  "data": [
    {
      "name": "primary",
      "values": [
        { "x": 0, "y": 14 },
        { "x": 1, "y": 18 },
        { "x": 2, "y": 48 },
        { "x": 3, "y": 44 },
        { "x": 4, "y": 27 },
        { "x": 5, "y": 26 },
        { "x": 6, "y": 12 },
        { "x": 7, "y": 31 },
        { "x": 8, "y": 22 },
        { "x": 9, "y": 4 },
        { "x": 10, "y": 2 },
        { "x": 11, "y": 37 },
        { "x": 12, "y": 48 },
        { "x": 13, "y": 22 },
        { "x": 14, "y": 8 },
        { "x": 15, "y": 2 },
        { "x": 16, "y": 27 },
        { "x": 17, "y": 26 },
        { "x": 18, "y": 24 },
        { "x": 19, "y": 6 },
        { "x": 20, "y": 44 },
        { "x": 21, "y": 48 },
        { "x": 22, "y": 1 },
        { "x": 23, "y": 49 },
        { "x": 24, "y": 33 },
        { "x": 25, "y": 2 }
       ]
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
          "width": 50.0,
          "height": { "field": "y", "scale": "vertical" },
          "fill": "blue"
        }
      }
    ]
  }
}`;

const bruc = Bruc.build(spec);

await bruc.renderAsSvg("#first");

/*while(true) {
  const data = randomData();
  console.log(JSON.stringify(data));
  await bruc.setData("primary", data);

  await delay(1000);
}*/

function randomData() {
  const values = [];
  for (let i = 0; i <= 25; i++) {
    values.push({ x: i, y: randomValue(50) });
  }
  return values;
}

function randomValue(max) {
  return Math.floor(Math.random() * max);
}

function delay(time) {
  return new Promise(resolve => setTimeout(resolve, time));
}


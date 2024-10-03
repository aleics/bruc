import { Bruc } from "bruc";

const linearSpec = `{
  "dimensions": {
    "width": 600,
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
      "domain": { "data": "primary", "field": "x" },
      "range": [0, 600]
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
      },
      {
        "from": "primary",
        "type": "line",
        "properties": {
          "x": { "field": "a", "scale": "horizontal" },
          "y": { "field": "b", "scale": "vertical" },
          "stroke": "green",
          "strokeWidth": 1.5
        }
      }
    ]
  }
}`;

const logSpec = `{
  "dimensions": {
    "width": 600,
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
      "type": "log",
      "name": "horizontal",
      "domain": { "data": "primary", "field": "x" },
      "range": [0, 600]
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
      },
      {
        "from": "primary",
        "type": "line",
        "properties": {
          "x": { "field": "a", "scale": "horizontal" },
          "y": { "field": "b", "scale": "vertical" },
          "stroke": "green",
          "strokeWidth": 1.5
        }
      }
    ]
  }
}`;

const linear = Bruc.build(linearSpec);

await linear.renderAsSvg("#first");
await linear.renderAsSvg("#second");

const log = Bruc.build(logSpec);

await log.renderAsSvg("#first-log");
await log.renderAsSvg("#second-log");

while (true) {
  const data = randomData();
  await linear.setData("primary", data);
  await log.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 1; i <= randomValue(0, 200); i++) {
    values.push({
      x: i,
      y: randomValue(0, 50),
      k: i,
      q: randomValue(0, 50),
      a: i,
      b: randomValue(0, 50),
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

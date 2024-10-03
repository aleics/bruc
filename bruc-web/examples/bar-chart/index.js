import { Bruc } from "bruc";

const specHorizontal = `{
  "dimensions": {
    "width": 600,
    "height": 200
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
      "range": [0, 600]
    },
    {
      "type": "linear",
      "name": "vertical",
      "domain": { "data": "primary", "field": "q" },
      "range": [0, 200]
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

const specHorizontalLog = `{
  "dimensions": {
    "width": 600,
    "height": 200
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
      "range": [0, 600]
    },
    {
      "type": "log",
      "name": "vertical",
      "domain": { "data": "primary", "field": "q" },
      "range": [0, 200]
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
          "fill": "green"
        }
      }
    ]
  }
}`;

const specVertical = `{
  "dimensions": {
    "width": 600,
    "height": 400
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
      "range": [0, 600]
    },
    {
      "type": "band",
      "name": "vertical",
      "domain": { "data": "primary", "field": "k" },
      "range": [0, 400]
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

const specVerticalLog = `{
  "dimensions": {
    "width": 600,
    "height": 400
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
      "domain": { "data": "primary", "field": "q" },
      "range": [0, 600]
    },
    {
      "type": "band",
      "name": "vertical",
      "domain": { "data": "primary", "field": "k" },
      "range": [0, 400]
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
          "fill": "orange"
        }
      }
    ]
  }
}`;

const horizontal = Bruc.build(specHorizontal);
await horizontal.renderAsSvg("#first");

const horizontalLog = Bruc.build(specHorizontalLog);
await horizontalLog.renderAsSvg("#first-log");

const vertical = Bruc.build(specVertical);
await vertical.renderAsSvg("#second");

const verticalLog = Bruc.build(specVerticalLog);
await verticalLog.renderAsSvg("#second-log");

while (true) {
  const data = randomData();

  await horizontal.setData("primary", data);
  await vertical.setData("primary", data);
  await horizontalLog.setData("primary", data);
  await verticalLog.setData("primary", data);

  await delay(1000);
}

function randomData() {
  const values = [];
  for (let i = 1; i <= 20; i++) {
    const y = randomValue(1, 50);
    values.push({ k: i, q: y });
  }
  return values;
}

function randomValue(min, max) {
  return Math.random() * (max - min) + min;
}

function delay(time) {
  return new Promise((resolve) => setTimeout(resolve, time));
}

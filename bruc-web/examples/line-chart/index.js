import { Bruc } from "bruc";

const spec = `{
  "dimensions": {
    "width": 500,
    "height": 200
  },
  "data": [
    {
      "name": "primary",
      "values": [
        { "x": 0, "y": 0, "k": 0, "q": 10 },
        { "x": 1, "y": 50, "k": 1, "q": 20 },
        { "x": 2, "y": 15, "k": 2, "q": 50 },
        { "x": 3, "y": 30, "k": 3, "q": 10 }
      ]
    }
  ],
  "scales": [
    {
      "type": "linear",
      "name": "horizontal",
      "domain": [0, 5],
      "range": [0, 500]
    },
    {
      "type": "linear",
      "name": "vertical",
      "domain": [0, 50],
      "range": [0, 200]
    }
  ],
  "marks": [
    {
      "from": "primary",
      "type": "line",
      "on": {
        "update": {
          "x": { "field": "x", "scale": "horizontal" },
          "y": { "field": "y", "scale": "vertical" },
          "stroke": "red",
          "strokeWidth": 2
        }
      }
    },
    {
      "from": "primary",
      "type": "line",
      "on": {
        "update": {
          "x": { "field": "k", "scale": "horizontal" },
          "y": { "field": "q", "scale": "vertical" },
          "stroke": "blue",
          "strokeWidth": 2
        }
      }
    }
  ]
}`;

const bruc = await Bruc.build(spec);

bruc.renderAsSvg("#chart");

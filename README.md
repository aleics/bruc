# bruc
_bruc_ is a visualization library written in Rust :crab:

`bruc` is heavily inspired by [Vega](https://vega.github.io/vega/), a visualization grammar, which allows to create
visualization designs in a declarative way. This follows the principles presented by Leland Wilkinson in his book
[_The Grammar of Graphics_](https://link.springer.com/book/10.1007/0-387-28695-0).

At the time being, this project is a mere proof of concept to demonstrate, how such a library could be implemented using
[Web Assembly](https://webassembly.org/). And thus, be used in different environments with a native feel.

## Build
To build the project for `wasm`, you can do that by running [`wasm-pack`](https://github.com/rustwasm/wasm-pack) under the `bruc-wasm` project:

```shell
cd bruc-wasm/
wasm-pack build --target web
```

Then, you can build the `bruc-web` project:

```shell
cd bruc-web/
npm run build
```

And run the examples:

```shell
cd bruc-web/examples/line-chart
npm i
npm start
```

## Demo
To see `bruc` in action, refer to the [`bruc-web` examples](https://github.com/aleics/bruc/tree/main/bruc-web/examples),
or directly the [`bruc-core` examples](https://github.com/aleics/bruc/tree/main/bruc-core/examples).

## Usage
### Primitives
`bruc` makes use of a well-defined specification to declare and design the different parts of the visualization and
how those elements, named also _primitives_, should interact together. There's three main elements:
 - `data`: defines the different input data sources its respective values.
 - `visual`: defines what visual elements exist in the visualization. This is mainly divided by axes and any kind of
    shape (e.g. lines or bars).
 - `scale`: defines how to map the input data values into the coordinates of the canvas that the visual elements are
    placed.

### Renders
`bruc` supports at the moment only SVG as a render artifact. This can be extended in the future to other type of
renderers

### Basic example
Following, there's an example on how to define a specification for a simple line chart, and how to use `bruc` to visualize in
the web:

```ts
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
        { "x": 9, "y": 4 }
      ]
    }
  ],
  "scales": [
    {
      "type": "linear",
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
        "orientation": "bottom",
        "scale": "horizontal"
      },
      {
        "orientation": "left",
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
      }
    ]
  }
}`

// Parse and validate the specification and build the internal View
const bruc = Bruc.build(spec);

// Render the visualization by running the input data over the View
// Once the SVG is rendered, attached to the dom element identified by the selector
await bruc.renderAsSvg("#first");
```

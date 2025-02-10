# Ramer-Douglas-Peucker (RDP) Python Package

This Python package provides an efficient implementation of the Ramer-Douglas-Peucker algorithm using Rust and PyO3. The algorithm simplifies a curve composed of line segments by reducing the number of points while preserving the overall shape.

This implementation is a port of [fhirschmann/rdp](https://github.com/fhirschmann/rdp/tree/master).

## Installation

To install the package, run:

```sh
pip install git+https://github.com/salvomcl/rdprs.git
```

## Usage

```python
from rdprs import rdp_simplify

points = [
    (0.0, 0.0, 0.0),
    (1.0, 0.1, 0.0),
    (2.0, -0.1, 0.0),
    (3.0, 0.0, 0.0)
]
epsilon = 0.2

mask = rdp_simplify(points, epsilon)
print(mask)  # Example output: [True, False, False, True]
```

## Parameters
- `points`: A list of tuples representing the (x, y, z) coordinates of the original curve.
- `epsilon`: A float representing the maximum distance a point can deviate before being considered essential.

## Features
- **High Performance**: Implemented in Rust for speed.
- **Easy to Use**: Simple Python interface.
- **Accurate**: Preserves important features of the curve while reducing complexity.

## License

This project is licensed under the MIT License.

## Contributing

Pull requests and issues are welcome! Feel free to improve the package or report bugs.

## Acknowledgments

- [Ramer-Douglas-Peucker Algorithm](https://en.wikipedia.org/wiki/Ramer-Douglas-Peucker_algorithm)
- Built with [PyO3](https://pyo3.rs/) and [Maturin](https://github.com/PyO3/maturin)
- Based on [fhirschmann/rdp](https://github.com/fhirschmann/rdp/tree/master)


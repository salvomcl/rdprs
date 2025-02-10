use pyo3::prelude::*;
use rdp::{rdp_simplify_mask, Point};

mod rdp;

/// Simplifies a 3D polyline using the Ramer-Douglas-Peucker algorithm.
///
/// # Arguments
/// * `points` - A vector of 3D points represented as `[f64; 3]` arrays.
/// * `epsilon` - The tolerance level for simplification; higher values yield more simplification.
///
/// # Returns
/// A `Vec<bool>` where `true` indicates that the corresponding point is retained.
///
/// # Example (Python Usage)
/// ```python
/// import rdprs
/// result = rdprs.rdp_simplify([(0.0, 0.0, 0.0), (1.0, 0.1, 0.0), (2.0, -0.1, 0.0), (3.0, 0.0, 0.0)], 0.15)
/// print(result)  # Example output: [True, False, False, True]
/// ```
#[pyfunction]
fn rdp_simplify(points: Vec<[f64; 3]>, epsilon: f64) -> Vec<bool> {
    let points: Vec<Point> = points
        .iter()
        .map(|p| Point::new(p[0], p[1], p[2]))
        .collect();
    rdp_simplify_mask(&points, epsilon)
}

/// Python module definition for `rdprs`.
///
/// This module provides a Python interface for the Ramer-Douglas-Peucker simplification algorithm.
///
/// # Functions
/// * `rdp_simplify(points: list[tuple[float, float, float]], epsilon: float) -> List[bool]` - Simplifies a 3D polyline.
#[pymodule]
fn rdprs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rdp_simplify, m)?)?;
    Ok(())
}

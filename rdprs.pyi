def rdp_simplify(
    points: list[tuple[float, float, float]], epsilon: float
) -> list[bool]:
    """Simplifies a 3D polyline using the Ramer-Douglas-Peucker algorithm.

    This function takes a list of 3D points and an epsilon value, then returns a
    boolean mask indicating which points should be retained to form a simplified
    version of the polyline.

    Args:
        points (list[tuple[float, float, float]]): A list of points, where each
            point is represented as a tuple of three floats (x, y, z).
        epsilon (float): The distance threshold for simplification. A higher
            epsilon results in more aggressive simplification.

    Returns:
        list[bool]: A list of boolean values where `True` means the point is kept,
        and `False` means the point is removed.

    Examples:
        ```python
        import rdprs

        points = [
            (0.0, 0.0, 0.0),
            (1.0, 0.1, 0.0),
            (2.0, -0.1, 0.0),
            (3.0, 0.0, 0.0)
        ]
        epsilon = 0.2

        mask = rdprs.rdp_simplify(points, epsilon)
        print(mask)  # Example output: [True, False, False, True]
        ```
    """

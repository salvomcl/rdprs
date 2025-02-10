from rdprs import rdp_simplify


def test_two():
    """Point sequence with only two elements."""
    assert rdp_simplify([(0, 0, 0), (4, 4, 4)], 0) == [True, True]


def test_hor():
    """Horizontal line."""
    assert rdp_simplify(
        [(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0)],
        0,
    ) == [True, False, False, False, True]


def test_ver():
    """Vertical line."""
    assert rdp_simplify([(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0), (0, 4, 0)], 0) == [
        True,
        False,
        False,
        False,
        True,
    ]


def test_diag():
    """Diagonal line."""
    assert rdp_simplify([(0, 0, 0), (1, 1, 1), (2, 2, 2), (3, 3, 3), (4, 4, 4)], 0) == [
        True,
        False,
        False,
        False,
        True,
    ]


def test_eps0():
    """Epsilon being too small to be simplified."""
    assert rdp_simplify([(0, 0, 0), (5, 1, 0), (10, 1, 0)], 0) == [True, True, True]


def test_eps1():
    """Epsilon large enough to be simplified."""
    assert rdp_simplify([(0, 0, 0), (5, 1, 0), (10, 1, 0)], 1) == [True, False, True]


def test_L():
    """Point sequence which has the form of an L."""
    assert rdp_simplify([(5, 0, 0), (4, 0, 0), (3, 0, 0), (3, 1, 0), (3, 2, 0)], 0) == [
        True,
        False,
        True,
        False,
        True,
    ]

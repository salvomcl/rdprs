#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn squared_norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn scale(&self, scalar: f64) -> Point {
        Point::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

#[derive(Debug)]
struct LineSegment {
    a: Point,
    b: Point,
    ab: Point,
    len2: f64,
    inv_len2: f64,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> Self {
        let ab = b.sub(&a);
        let len2 = ab.squared_norm();
        let inv_len2 = 1.0 / len2;
        LineSegment {
            a,
            b,
            ab,
            len2,
            inv_len2,
        }
    }

    fn distance2(&self, p: &Point) -> f64 {
        let dot = p.sub(&self.a).dot(&self.ab);
        if dot <= 0.0 {
            p.sub(&self.a).squared_norm()
        } else if dot >= self.len2 {
            p.sub(&self.b).squared_norm()
        } else {
            self.a
                .add(&self.ab.scale(dot * self.inv_len2))
                .sub(p)
                .squared_norm()
        }
    }
}

fn rdp_simplify_iter(coords: &[Point], to_keep: &mut Vec<bool>, epsilon: f64) {
    let mut stack = Vec::new();
    stack.push((0, to_keep.len() - 1));
    while let Some((i, j)) = stack.pop() {
        to_keep[i] = true;
        to_keep[j] = true;
        if j - i <= 1 {
            continue;
        }
        let line = LineSegment::new(coords[i].clone(), coords[j].clone());
        let mut max_dist2 = 0.0;
        let mut max_index = i;
        for k in i + 1..j {
            let dist2 = line.distance2(&coords[k]);
            if dist2 > max_dist2 {
                max_dist2 = dist2;
                max_index = k;
            }
        }
        if max_dist2 <= epsilon * epsilon {
            continue;
        }
        stack.push((i, max_index));
        stack.push((max_index, j));
    }
}

pub fn rdp_simplify_mask(coords: &[Point], epsilon: f64) -> Vec<bool> {
    let mut mask = vec![false; coords.len()];
    rdp_simplify_iter(coords, &mut mask, epsilon);
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_operations() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let v2 = Point::new(4.0, 5.0, 6.0);

        assert_eq!(v1.add(&v2), Point::new(5.0, 7.0, 9.0));
        assert_eq!(v1.sub(&v2), Point::new(-3.0, -3.0, -3.0));
        assert_eq!(v1.scale(2.0), Point::new(2.0, 4.0, 6.0));
        assert_eq!(v1.dot(&v2), 32.0);
        assert_eq!(v1.squared_norm(), 14.0);
    }

    #[test]
    fn test_line_segment_distance() {
        let a = Point::new(0.0, 0.0, 0.0);
        let b = Point::new(1.0, 0.0, 0.0);
        let segment = LineSegment::new(a.clone(), b.clone());

        let p1 = Point::new(0.5, 1.0, 0.0);
        let p2 = Point::new(2.0, 0.0, 0.0);
        let p3 = Point::new(-1.0, 0.0, 0.0);

        assert_eq!(segment.distance2(&p1), 1.0);
        assert_eq!(segment.distance2(&p2), 1.0);
        assert_eq!(segment.distance2(&p3), 1.0);
    }

    #[test]
    fn test_rdp_simplify_mask() {
        let coords = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.1, 0.0),
            Point::new(2.0, -0.1, 0.0),
            Point::new(3.0, 0.0, 0.0),
        ];
        let epsilon = 0.15;
        let mask = rdp_simplify_mask(&coords, epsilon);

        assert_eq!(mask, vec![true, false, false, true]);
    }
}

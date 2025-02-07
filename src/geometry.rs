use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use tracing::debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Point2D<T> {
    pub x: f64,
    pub y: f64,
    pub data: Option<T>,
}

impl<T> Point2D<T> {
    pub fn new(x: f64, y: f64, data: Option<T>) -> Self {
        let pt = Self { x, y, data };
        debug!("Point2D::new() -> x: {}, y: {}", pt.x, pt.y);
        pt
    }

    pub fn distance_sq(&self, other: &Point2D<T>) -> f64 {
        let dist = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        debug!(
            "Point2D::distance_sq(): self: (x: {}, y: {}), other: (x: {}, y: {}), result: {}",
            self.x, self.y, other.x, other.y, dist
        );
        dist
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn contains<T>(&self, point: &Point2D<T>) -> bool {
        let res = point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height;
        debug!("Rectangle::contains(): self: (x: {}, y: {}, w: {}, h: {}), point: (x: {}, y: {}), result: {}", self.x, self.y, self.width, self.height, point.x, point.y, res);
        res
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        let res = !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y);
        debug!("Rectangle::intersects(): self: (x: {}, y: {}, w: {}, h: {}), other: (x: {}, y: {}, w: {}, h: {}), result: {}", self.x, self.y, self.width, self.height, other.x, other.y, other.width, other.height, res);
        res
    }

    pub fn area(&self) -> f64 {
        let area = self.width * self.height;
        debug!(
            "Rectangle::area(): (w: {}, h: {}) -> {}",
            self.width, self.height, area
        );
        area
    }

    pub fn union(&self, other: &Rectangle) -> Rectangle {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        let union_rect = Rectangle {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        };
        debug!("Rectangle::union(): self: (x: {}, y: {}, w: {}, h: {}), other: (x: {}, y: {}, w: {}, h: {}), result: (x: {}, y: {}, w: {}, h: {})", self.x, self.y, self.width, self.height, other.x, other.y, other.width, other.height, union_rect.x, union_rect.y, union_rect.width, union_rect.height);
        union_rect
    }

    pub fn enlargement(&self, other: &Rectangle) -> f64 {
        let extra = self.union(other).area() - self.area();
        debug!(
            "Rectangle::enlargement(): self area: {}, union area: {}, enlargement: {}",
            self.area(),
            self.union(other).area(),
            extra
        );
        extra
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3D<T> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub data: Option<T>,
}

impl<T> Point3D<T> {
    pub fn new(x: f64, y: f64, z: f64, data: Option<T>) -> Self {
        let pt = Self { x, y, z, data };
        debug!("Point3D::new() -> x: {}, y: {}, z: {}", pt.x, pt.y, pt.z);
        pt
    }

    pub fn distance_sq(&self, other: &Point3D<T>) -> f64 {
        let dist =
            (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2);
        debug!("Point3D::distance_sq(): self: (x: {}, y: {}, z: {}), other: (x: {}, y: {}, z: {}), result: {}", self.x, self.y, self.z, other.x, other.y, other.z, dist);
        dist
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl Cube {
    pub fn contains<T>(&self, point: &Point3D<T>) -> bool {
        let res = point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
            && point.z >= self.z
            && point.z <= self.z + self.depth;
        debug!("Cube::contains(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), point: (x: {}, y: {}, z: {}), result: {}", self.x, self.y, self.z, self.width, self.height, self.depth, point.x, point.y, point.z, res);
        res
    }

    pub fn intersects(&self, other: &Cube) -> bool {
        let res = !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y
            || other.z > self.z + self.depth
            || other.z + other.depth < self.z);
        debug!("Cube::intersects(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), other: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), result: {}", self.x, self.y, self.z, self.width, self.height, self.depth, other.x, other.y, other.z, other.width, other.height, other.depth, res);
        res
    }

    pub fn area(&self) -> f64 {
        let vol = self.width * self.height * self.depth;
        debug!(
            "Cube::area(): (w: {}, h: {}, d: {}) -> {}",
            self.width, self.height, self.depth, vol
        );
        vol
    }

    pub fn union(&self, other: &Cube) -> Cube {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let z1 = self.z.min(other.z);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        let z2 = (self.z + self.depth).max(other.z + other.depth);
        let union_cube = Cube {
            x: x1,
            y: y1,
            z: z1,
            width: x2 - x1,
            height: y2 - y1,
            depth: z2 - z1,
        };
        debug!("Cube::union(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), other: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), result: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {})", self.x, self.y, self.z, self.width, self.height, self.depth, other.x, other.y, other.z, other.width, other.height, other.depth, union_cube.x, union_cube.y, union_cube.z, union_cube.width, union_cube.height, union_cube.depth);
        union_cube
    }

    pub fn enlargement(&self, other: &Cube) -> f64 {
        let extra = self.union(other).area() - self.area();
        debug!(
            "Cube::enlargement(): self area: {}, union area: {}, enlargement: {}",
            self.area(),
            self.union(other).area(),
            extra
        );
        extra
    }
}

pub trait BSPBounds {
    const DIM: usize;
    fn center(&self, dim: usize) -> f64;
    fn extent(&self, dim: usize) -> f64;
}

impl BSPBounds for Rectangle {
    const DIM: usize = 2;
    fn center(&self, dim: usize) -> f64 {
        let center = match dim {
            0 => self.x + self.width / 2.0,
            1 => self.y + self.height / 2.0,
            _ => panic!("Rectangle only has dimensions 0 and 1"),
        };
        debug!("Rectangle::center(): dim: {}, center: {}", dim, center);
        center
    }
    fn extent(&self, dim: usize) -> f64 {
        let extent = match dim {
            0 => self.width,
            1 => self.height,
            _ => panic!("Rectangle only has dimensions 0 and 1"),
        };
        debug!("Rectangle::extent(): dim: {}, extent: {}", dim, extent);
        extent
    }
}

impl BSPBounds for Cube {
    const DIM: usize = 3;
    fn center(&self, dim: usize) -> f64 {
        let center = match dim {
            0 => self.x + self.width / 2.0,
            1 => self.y + self.height / 2.0,
            2 => self.z + self.depth / 2.0,
            _ => panic!("Cube only has dimensions 0, 1, and 2"),
        };
        debug!("Cube::center(): dim: {}, center: {}", dim, center);
        center
    }
    fn extent(&self, dim: usize) -> f64 {
        let extent = match dim {
            0 => self.width,
            1 => self.height,
            2 => self.depth,
            _ => panic!("Cube only has dimensions 0, 1, and 2"),
        };
        debug!("Cube::extent(): dim: {}, extent: {}", dim, extent);
        extent
    }
}

pub trait BoundingVolume: Clone {
    fn area(&self) -> f64;
    fn union(&self, other: &Self) -> Self;
    fn enlargement(&self, other: &Self) -> f64 {
        self.union(other).area() - self.area()
    }
    fn intersects(&self, other: &Self) -> bool;
}

impl BoundingVolume for Rectangle {
    fn area(&self) -> f64 {
        let a = self.area();
        debug!("BoundingVolume (Rectangle)::area() -> {}", a);
        a
    }
    fn union(&self, other: &Self) -> Self {
        let u = self.union(other);
        debug!("BoundingVolume (Rectangle)::union() computed.");
        u
    }
    fn intersects(&self, other: &Self) -> bool {
        let i = self.intersects(other);
        debug!("BoundingVolume (Rectangle)::intersects() -> {}", i);
        i
    }
}

impl BoundingVolume for Cube {
    fn area(&self) -> f64 {
        let a = self.area();
        debug!("BoundingVolume (Cube)::area() -> {}", a);
        a
    }
    fn union(&self, other: &Self) -> Self {
        let u = self.union(other);
        debug!("BoundingVolume (Cube)::union() computed.");
        u
    }
    fn intersects(&self, other: &Self) -> bool {
        let i = self.intersects(other);
        debug!("BoundingVolume (Cube)::intersects() -> {}", i);
        i
    }
}

#[derive(Debug)]
pub struct HeapItem<T: Clone> {
    pub neg_distance: OrderedFloat<f64>,
    pub point_2d: Option<Point2D<T>>,
    pub point_3d: Option<Point3D<T>>,
}

impl<T: Clone> PartialEq for HeapItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.neg_distance == other.neg_distance
    }
}

impl<T: Clone> Eq for HeapItem<T> {}

impl<T: Clone> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.neg_distance.partial_cmp(&other.neg_distance)
    }
}

impl<T: Clone> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.neg_distance.cmp(&other.neg_distance)
    }
}

pub trait HasMinDistance<Q> {
    fn min_distance(&self, query: &Q) -> f64;
}

pub trait BoundingVolumeFromPoint<Q>: BoundingVolume {
    fn from_point_radius(query: &Q, radius: f64) -> Self;
}

impl<T> HasMinDistance<Point2D<T>> for Rectangle {
    fn min_distance(&self, point: &Point2D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        (dx * dx + dy * dy).sqrt()
    }
}

impl<T> BoundingVolumeFromPoint<Point2D<T>> for Rectangle {
    fn from_point_radius(query: &Point2D<T>, radius: f64) -> Self {
        Rectangle {
            x: query.x - radius,
            y: query.y - radius,
            width: 2.0 * radius,
            height: 2.0 * radius,
        }
    }
}

impl<T> HasMinDistance<Point3D<T>> for Cube {
    fn min_distance(&self, point: &Point3D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        let dz = if point.z < self.z {
            self.z - point.z
        } else if point.z > self.z + self.depth {
            point.z - (self.z + self.depth)
        } else {
            0.0
        };
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl<T> BoundingVolumeFromPoint<Point3D<T>> for Cube {
    fn from_point_radius(query: &Point3D<T>, radius: f64) -> Self {
        Cube {
            x: query.x - radius,
            y: query.y - radius,
            z: query.z - radius,
            width: 2.0 * radius,
            height: 2.0 * radius,
            depth: 2.0 * radius,
        }
    }
}

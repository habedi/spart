#[derive(Debug, Clone, PartialEq)]
pub struct Point2D<T> {
    pub x: f64,
    pub y: f64,
    pub data: Option<T>,
}

impl<T> Point2D<T> {
    pub fn new(x: f64, y: f64, data: Option<T>) -> Self {
        Point2D { x, y, data }
    }

    pub fn distance_sq(&self, other: &Point2D<T>) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
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
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y)
    }

    /// Returns the area of the rectangle.
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Returns the smallest rectangle that contains both `self` and `other`.
    pub fn union(&self, other: &Rectangle) -> Rectangle {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        Rectangle {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    /// Returns the additional area required to enlarge `self` to include `other`.
    pub fn enlargement(&self, other: &Rectangle) -> f64 {
        self.union(other).area() - self.area()
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
        Point3D { x, y, z, data }
    }

    pub fn distance_sq(&self, other: &Point3D<T>) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
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
    /// Returns true if the cube contains the given 3D point.
    pub fn contains<T>(&self, point: &Point3D<T>) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
            && point.z >= self.z
            && point.z <= self.z + self.depth
    }

    /// Returns true if this cube intersects another cube.
    pub fn intersects(&self, other: &Cube) -> bool {
        !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y
            || other.z > self.z + self.depth
            || other.z + other.depth < self.z)
    }
}

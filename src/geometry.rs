//! ## Geometric Primitives and Operations for 2D and 3D Spaces
//!
//! This module provides geometric primitives and operations for both 2D and 3D spaces.
//! It defines types such as `Point2D`, `Rectangle`, `Point3D`, and `Cube` along with their associated
//! operations. These types form the basis for spatial indexing and query algorithms used in Spart.
//!
//! In addition to the basic types, the module defines several traits for operations such as
//! bounding volume calculations and minimum distance computations.

use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use tracing::debug;

// Import custom errors from the exceptions module.
use crate::exceptions::SpartError;

/// Represents a 2D point with an optional payload.
///
/// ### Example
///
/// ```
/// use spart::geometry::Point2D;
/// // Use an explicit type parameter (here, `()`) so that the type can be inferred.
/// let pt: Point2D<()> = Point2D::new(1.0, 2.0, None);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D<T> {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
    /// Optional associated data.
    pub data: Option<T>,
}

impl<T> Point2D<T> {
    /// Creates a new `Point2D` with the given coordinates and optional data.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    /// * `data` - Optional data associated with the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Point2D;
    /// let pt: Point2D<()> = Point2D::new(1.0, 2.0, None);
    /// ```
    pub fn new(x: f64, y: f64, data: Option<T>) -> Self {
        let pt = Self { x, y, data };
        debug!("Point2D::new() -> x: {}, y: {}", pt.x, pt.y);
        pt
    }

    /// Computes the squared Euclidean distance between this point and another.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Point2D;
    /// let a: Point2D<()> = Point2D::new(0.0, 0.0, None);
    /// let b: Point2D<()> = Point2D::new(3.0, 4.0, None);
    /// assert_eq!(a.distance_sq(&b), 25.0);
    /// ```
    pub fn distance_sq(&self, other: &Point2D<T>) -> f64 {
        let dist = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        debug!(
            "Point2D::distance_sq(): self: (x: {}, y: {}), other: (x: {}, y: {}), result: {}",
            self.x, self.y, other.x, other.y, dist
        );
        dist
    }
}

/// Represents a rectangle in 2D space.
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// The x-coordinate of the rectangle's top-left corner.
    pub x: f64,
    /// The y-coordinate of the rectangle's top-left corner.
    pub y: f64,
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
}

impl Rectangle {
    /// Determines if the rectangle contains the given point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to test.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::{Rectangle, Point2D};
    /// let rect = Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
    /// let pt: Point2D<()> = Point2D::new(5.0, 5.0, None);
    /// assert!(rect.contains(&pt));
    /// ```
    pub fn contains<T>(&self, point: &Point2D<T>) -> bool {
        let res = point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height;
        debug!("Rectangle::contains(): self: (x: {}, y: {}, w: {}, h: {}), point: (x: {}, y: {}), result: {}",
            self.x, self.y, self.width, self.height, point.x, point.y, res);
        res
    }

    /// Determines whether this rectangle intersects with another.
    ///
    /// # Arguments
    ///
    /// * `other` - The other rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Rectangle;
    /// let a = Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 };
    /// let b = Rectangle { x: 5.0, y: 5.0, width: 10.0, height: 10.0 };
    /// assert!(a.intersects(&b));
    /// ```
    pub fn intersects(&self, other: &Rectangle) -> bool {
        let res = !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y);
        debug!("Rectangle::intersects(): self: (x: {}, y: {}, w: {}, h: {}), other: (x: {}, y: {}, w: {}, h: {}), result: {}",
            self.x, self.y, self.width, self.height, other.x, other.y, other.width, other.height, res);
        res
    }

    /// Computes the area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Rectangle;
    /// let rect = Rectangle { x: 0.0, y: 0.0, width: 4.0, height: 5.0 };
    /// assert_eq!(rect.area(), 20.0);
    /// ```
    pub fn area(&self) -> f64 {
        let area = self.width * self.height;
        debug!(
            "Rectangle::area(): (w: {}, h: {}) -> {}",
            self.width, self.height, area
        );
        area
    }

    /// Computes the union of this rectangle with another.
    ///
    /// The union is defined as the smallest rectangle that completely contains both rectangles.
    ///
    /// # Arguments
    ///
    /// * `other` - The other rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Rectangle;
    /// let a = Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 };
    /// let b = Rectangle { x: 3.0, y: 3.0, width: 5.0, height: 5.0 };
    /// let union_rect = a.union(&b);
    /// assert_eq!(union_rect.x, 0.0);
    /// ```
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
        debug!("Rectangle::union(): self: (x: {}, y: {}, w: {}, h: {}), other: (x: {}, y: {}, w: {}, h: {}), result: (x: {}, y: {}, w: {}, h: {})",
            self.x, self.y, self.width, self.height, other.x, other.y, other.width, other.height,
            union_rect.x, union_rect.y, union_rect.width, union_rect.height);
        union_rect
    }

    /// Computes the enlargement needed to include another rectangle.
    ///
    /// The enlargement is defined as the difference between the area of the union and the area of this rectangle.
    ///
    /// # Arguments
    ///
    /// * `other` - The other rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Rectangle;
    /// let a = Rectangle { x: 0.0, y: 0.0, width: 4.0, height: 4.0 };
    /// let b = Rectangle { x: 2.0, y: 2.0, width: 4.0, height: 4.0 };
    /// let enlargement = a.enlargement(&b);
    /// assert!(enlargement >= 0.0);
    /// ```
    pub fn enlargement(&self, other: &Rectangle) -> f64 {
        let union_rect = self.union(other);
        let self_area = self.area();
        let union_area = union_rect.area();
        let extra = union_area - self_area;
        debug!(
            "Rectangle::enlargement(): self area: {}, union area: {}, enlargement: {}",
            self_area, union_area, extra
        );
        extra
    }
}

/// Represents a 3D point with an optional payload.
///
/// # Examples
///
/// ```
/// use spart::geometry::Point3D;
/// let pt: Point3D<()> = Point3D::new(1.0, 2.0, 3.0, None);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D<T> {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
    /// The z-coordinate of the point.
    pub z: f64,
    /// Optional associated data.
    pub data: Option<T>,
}

impl<T> Point3D<T> {
    /// Creates a new `Point3D` with the given coordinates and optional data.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate.
    /// * `y` - The y-coordinate.
    /// * `z` - The z-coordinate.
    /// * `data` - Optional data associated with the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Point3D;
    /// let pt: Point3D<()> = Point3D::new(1.0, 2.0, 3.0, None);
    /// ```
    pub fn new(x: f64, y: f64, z: f64, data: Option<T>) -> Self {
        let pt = Self { x, y, z, data };
        debug!("Point3D::new() -> x: {}, y: {}, z: {}", pt.x, pt.y, pt.z);
        pt
    }

    /// Computes the squared Euclidean distance between this point and another.
    ///
    /// # Arguments
    ///
    /// * `other` - The other 3D point.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Point3D;
    /// let a: Point3D<()> = Point3D::new(0.0, 0.0, 0.0, None);
    /// let b: Point3D<()> = Point3D::new(1.0, 2.0, 2.0, None);
    /// assert_eq!(a.distance_sq(&b), 9.0);
    /// ```
    pub fn distance_sq(&self, other: &Point3D<T>) -> f64 {
        let dist =
            (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2);
        debug!("Point3D::distance_sq(): self: (x: {}, y: {}, z: {}), other: (x: {}, y: {}, z: {}), result: {}",
            self.x, self.y, self.z, other.x, other.y, other.z, dist);
        dist
    }
}

/// Represents a cube (or cuboid) in 3D space.
#[derive(Debug, Clone)]
pub struct Cube {
    /// The x-coordinate of the cube's top-left-front corner.
    pub x: f64,
    /// The y-coordinate of the cube's top-left-front corner.
    pub y: f64,
    /// The z-coordinate of the cube's top-left-front corner.
    pub z: f64,
    /// The width of the cube.
    pub width: f64,
    /// The height of the cube.
    pub height: f64,
    /// The depth of the cube.
    pub depth: f64,
}

impl Cube {
    /// Determines if the cube contains the given 3D point.
    ///
    /// # Arguments
    ///
    /// * `point` - The 3D point to test.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::{Cube, Point3D};
    /// let cube = Cube { x: 0.0, y: 0.0, z: 0.0, width: 10.0, height: 10.0, depth: 10.0 };
    /// let pt: Point3D<()> = Point3D::new(5.0, 5.0, 5.0, None);
    /// assert!(cube.contains(&pt));
    /// ```
    pub fn contains<T>(&self, point: &Point3D<T>) -> bool {
        let res = point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
            && point.z >= self.z
            && point.z <= self.z + self.depth;
        debug!("Cube::contains(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), point: (x: {}, y: {}, z: {}), result: {}",
            self.x, self.y, self.z, self.width, self.height, self.depth,
            point.x, point.y, point.z, res);
        res
    }

    /// Determines whether this cube intersects with another cube.
    ///
    /// # Arguments
    ///
    /// * `other` - The other cube.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Cube;
    /// let a = Cube { x: 0.0, y: 0.0, z: 0.0, width: 5.0, height: 5.0, depth: 5.0 };
    /// let b = Cube { x: 3.0, y: 3.0, z: 3.0, width: 5.0, height: 5.0, depth: 5.0 };
    /// assert!(a.intersects(&b));
    /// ```
    pub fn intersects(&self, other: &Cube) -> bool {
        let res = !(other.x > self.x + self.width
            || other.x + other.width < self.x
            || other.y > self.y + self.height
            || other.y + other.height < self.y
            || other.z > self.z + self.depth
            || other.z + other.depth < self.z);
        debug!("Cube::intersects(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), other: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), result: {}",
            self.x, self.y, self.z, self.width, self.height, self.depth,
            other.x, other.y, other.z, other.width, other.height, other.depth, res);
        res
    }

    /// Computes the volume of the cube.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Cube;
    /// let cube = Cube { x: 0.0, y: 0.0, z: 0.0, width: 2.0, height: 3.0, depth: 4.0 };
    /// assert_eq!(cube.area(), 24.0);
    /// ```
    pub fn area(&self) -> f64 {
        let vol = self.width * self.height * self.depth;
        debug!(
            "Cube::area(): (w: {}, h: {}, d: {}) -> {}",
            self.width, self.height, self.depth, vol
        );
        vol
    }

    /// Computes the union of this cube with another.
    ///
    /// The union is defined as the smallest cube that completely contains both cubes.
    ///
    /// # Arguments
    ///
    /// * `other` - The other cube.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Cube;
    /// let a = Cube { x: 0.0, y: 0.0, z: 0.0, width: 3.0, height: 3.0, depth: 3.0 };
    /// let b = Cube { x: 2.0, y: 2.0, z: 2.0, width: 3.0, height: 3.0, depth: 3.0 };
    /// let union_cube = a.union(&b);
    /// assert_eq!(union_cube.x, 0.0);
    /// ```
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
        debug!("Cube::union(): self: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), other: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {}), result: (x: {}, y: {}, z: {}, w: {}, h: {}, d: {})",
            self.x, self.y, self.z, self.width, self.height, self.depth,
            other.x, other.y, other.z, other.width, other.height, other.depth,
            union_cube.x, union_cube.y, union_cube.z, union_cube.width, union_cube.height, union_cube.depth);
        union_cube
    }

    /// Computes the enlargement needed to include another cube.
    ///
    /// The enlargement is defined as the difference between the volume of the union and the volume of this cube.
    ///
    /// # Arguments
    ///
    /// * `other` - The other cube.
    ///
    /// # Examples
    ///
    /// ```
    /// use spart::geometry::Cube;
    /// let a = Cube { x: 0.0, y: 0.0, z: 0.0, width: 2.0, height: 2.0, depth: 2.0 };
    /// let b = Cube { x: 1.0, y: 1.0, z: 1.0, width: 2.0, height: 2.0, depth: 2.0 };
    /// let enlargement = a.enlargement(&b);
    /// assert!(enlargement >= 0.0);
    /// ```
    pub fn enlargement(&self, other: &Cube) -> f64 {
        let union_cube = self.union(other);
        let self_area = self.area();
        let union_area = union_cube.area();
        let extra = union_area - self_area;
        debug!(
            "Cube::enlargement(): self volume: {}, union volume: {}, enlargement: {}",
            self_area, union_area, extra
        );
        extra
    }
}

/// Trait for types that can provide the center and extent along a specified dimension.
///
/// # Panics
///
/// The methods in this trait will panic if an invalid dimension is provided. The panic messages are generated using `SpartError`.
pub trait BSPBounds {
    /// The number of dimensions supported.
    const DIM: usize;
    /// Returns the center coordinate along the specified dimension.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension index.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidDimension` if `dim` is not within the valid range.
    fn center(&self, dim: usize) -> f64;
    /// Returns the extent (width, height, or depth) along the specified dimension.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimension index.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidDimension` if `dim` is not within the valid range.
    fn extent(&self, dim: usize) -> f64;
}

impl BSPBounds for Rectangle {
    const DIM: usize = 2;
    fn center(&self, dim: usize) -> f64 {
        match dim {
            0 => self.x + self.width / 2.0,
            1 => self.y + self.height / 2.0,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: dim,
                    available: 2
                }
            ),
        }
    }
    fn extent(&self, dim: usize) -> f64 {
        match dim {
            0 => self.width,
            1 => self.height,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: dim,
                    available: 2
                }
            ),
        }
    }
}

impl BSPBounds for Cube {
    const DIM: usize = 3;
    fn center(&self, dim: usize) -> f64 {
        match dim {
            0 => self.x + self.width / 2.0,
            1 => self.y + self.height / 2.0,
            2 => self.z + self.depth / 2.0,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: dim,
                    available: 3
                }
            ),
        }
    }
    fn extent(&self, dim: usize) -> f64 {
        match dim {
            0 => self.width,
            1 => self.height,
            2 => self.depth,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: dim,
                    available: 3
                }
            ),
        }
    }
}

/// Trait representing a bounding volume, such as a rectangle or cube.
///
/// This trait abstracts common operations for geometric volumes used in spatial indexing.
pub trait BoundingVolume: Clone {
    /// Returns the area (or volume for 3D objects) of the bounding volume.
    fn area(&self) -> f64;
    /// Returns the smallest bounding volume that contains both `self` and `other`.
    fn union(&self, other: &Self) -> Self;
    /// Computes the enlargement required to include `other` in the bounding volume.
    ///
    /// By default, this is calculated as `union(other).area() - self.area()`.
    fn enlargement(&self, other: &Self) -> f64 {
        self.union(other).area() - self.area()
    }
    /// Determines whether the bounding volume intersects with another.
    fn intersects(&self, other: &Self) -> bool;
}

impl BoundingVolume for Rectangle {
    fn area(&self) -> f64 {
        let a = Rectangle::area(self);
        debug!("BoundingVolume (Rectangle)::area() -> {}", a);
        a
    }
    fn union(&self, other: &Self) -> Self {
        let u = Rectangle::union(self, other);
        debug!("BoundingVolume (Rectangle)::union() computed.");
        u
    }
    fn intersects(&self, other: &Self) -> bool {
        let i = Rectangle::intersects(self, other);
        debug!("BoundingVolume (Rectangle)::intersects() -> {}", i);
        i
    }
}

impl BoundingVolume for Cube {
    fn area(&self) -> f64 {
        let a = Cube::area(self);
        debug!("BoundingVolume (Cube)::area() -> {}", a);
        a
    }
    fn union(&self, other: &Self) -> Self {
        let u = Cube::union(self, other);
        debug!("BoundingVolume (Cube)::union() computed.");
        u
    }
    fn intersects(&self, other: &Self) -> bool {
        let i = Cube::intersects(self, other);
        debug!("BoundingVolume (Cube)::intersects() -> {}", i);
        i
    }
}

/// Represents an item in a heap, typically used for nearest neighbor or best-first search algorithms.
///
/// The `neg_distance` field is used to order items in a max-heap by their (negated) distance value.
#[derive(Debug)]
pub struct HeapItem<T: Clone> {
    /// The negated distance, used for ordering.
    pub neg_distance: OrderedFloat<f64>,
    /// An optional 2D point associated with the heap item.
    pub point_2d: Option<Point2D<T>>,
    /// An optional 3D point associated with the heap item.
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
        Some(self.cmp(other))
    }
}

impl<T: Clone> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.neg_distance.cmp(&other.neg_distance)
    }
}

/// Trait for types that can compute the minimum distance to a given query.
pub trait HasMinDistance<Q> {
    /// Computes the minimum distance from the bounding volume to the given query.
    fn min_distance(&self, query: &Q) -> f64;
}

/// Trait for constructing a bounding volume from a point and a radius.
pub trait BoundingVolumeFromPoint<Q>: BoundingVolume {
    /// Creates a bounding volume that encapsulates a point with the specified radius.
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

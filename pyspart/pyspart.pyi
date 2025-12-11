"""
Type stubs for PySpart - Python bindings for Spart Rust crate.

This file provides type hints for IDEs and type checkers.
"""

from typing import Optional, Dict, List, Tuple, Union, Any, Iterator, TypedDict, Type, TypeVar


# Basic geometry dictionaries accepted by constructors
class RectangleDict(TypedDict):
    """A rectangle boundary definition used by Quadtree.

    Keys:
        x: The x-coordinate of the rectangle's origin.
        y: The y-coordinate of the rectangle's origin.
        width: The width of the rectangle.
        height: The height of the rectangle.
    """
    x: float
    y: float
    width: float
    height: float


class CubeDict(TypedDict):
    """A cube boundary definition used by Octree.

    Keys:
        x: The x-coordinate of the cube's origin.
        y: The y-coordinate of the cube's origin.
        z: The z-coordinate of the cube's origin.
        width: The width of the cube.
        height: The height of the cube.
        depth: The depth of the cube.
    """
    x: float
    y: float
    z: float
    width: float
    height: float
    depth: float


class Point2D:
    """A 2D point with associated user data.

    Attributes:
        x: The x-coordinate.
        y: The y-coordinate.
        data: Arbitrary Python object associated with the point.
    """
    x: float
    y: float
    data: Any

    def __init__(self, x: float, y: float, data: Any) -> None:
        """Create a new 2D point.

        Args:
            x: X-coordinate.
            y: Y-coordinate.
            data: Arbitrary Python data to associate with the point.
        """
        ...

    def __eq__(self, other: object) -> bool:
        """Compare two points for equality (coordinates and data)."""
        ...

    # Not hashable in runtime; keep signature to avoid misleading type checkers
    def __hash__(self) -> int:
        """Hash is not supported; raises at runtime."""
        ...


class Point3D:
    """A 3D point with associated user data.

    Attributes:
        x: The x-coordinate.
        y: The y-coordinate.
        z: The z-coordinate.
        data: Arbitrary Python object associated with the point.
    """
    x: float
    y: float
    z: float
    data: Any

    def __init__(self, x: float, y: float, z: float, data: Any) -> None:
        """Create a new 3D point.

        Args:
            x: X-coordinate.
            y: Y-coordinate.
            z: Z-coordinate.
            data: Arbitrary Python data to associate with the point.
        """
        ...

    def __eq__(self, other: object) -> bool:
        """Compare two points for equality (coordinates and data)."""
        ...

    # Not hashable in runtime; keep signature to avoid misleading type checkers
    def __hash__(self) -> int:
        """Hash is not supported; raises at runtime."""
        ...


_TQT = TypeVar("_TQT", bound="Quadtree")
_TOT = TypeVar("_TOT", bound="Octree")
_TK2 = TypeVar("_TK2", bound="KdTree2D")
_TK3 = TypeVar("_TK3", bound="KdTree3D")
_TR2 = TypeVar("_TR2", bound="RTree2D")
_TR3 = TypeVar("_TR3", bound="RTree3D")
_TRS2 = TypeVar("_TRS2", bound="RStarTree2D")
_TRS3 = TypeVar("_TRS3", bound="RStarTree3D")


class Quadtree:
    """A 2D spatial index dividing space into quadrants.

    Stores 2D points and supports nearest-neighbor and range queries
    using Euclidean distance.
    """

    def __init__(self, boundary: RectangleDict, capacity: int) -> None:
        """Create a quadtree with a rectangular boundary.

        Args:
            boundary: A rectangle dict specifying the root boundary.
            capacity: Max points per node before subdivision.
        """
        ...

    def insert(self, point: Point2D) -> bool:
        """Insert a point into the tree.

        Returns:
            True if insertion succeeded, False otherwise.
        """
        ...

    def insert_bulk(self, points: List[Point2D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point2D) -> bool:
        """Delete a point from the tree.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point2D, k: int) -> List[Point2D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point2D, radius: float) -> List[Point2D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TQT], path: str) -> _TQT:
        """Load a quadtree from a file path."""
        ...


class Octree:
    """A 3D spatial index dividing space into octants.

    Stores 3D points and supports nearest-neighbor and range queries
    using Euclidean distance.
    """

    def __init__(self, boundary: CubeDict, capacity: int) -> None:
        """Create an octree with a cubic boundary.

        Args:
            boundary: A cube dict specifying the root boundary.
            capacity: Max points per node before subdivision.
        """
        ...

    def insert(self, point: Point3D) -> bool:
        """Insert a point into the tree.

        Returns:
            True if insertion succeeded, False otherwise.
        """
        ...

    def insert_bulk(self, points: List[Point3D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point3D) -> bool:
        """Delete a point from the tree.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point3D, k: int) -> List[Point3D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point3D, radius: float) -> List[Point3D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TOT], path: str) -> _TOT:
        """Load an octree from a file path."""
        ...


class KdTree2D:
    """A k-d tree for 2D points supporting NN and range queries.

    Uses Euclidean distance for searches.
    """

    def __init__(self) -> None:
        """Create an empty 2D k-d tree."""
        ...

    def insert(self, point: Point2D) -> None:
        """Insert a point; raises ValueError on invalid input."""
        ...

    def insert_bulk(self, points: List[Point2D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point2D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point2D, k: int) -> List[Point2D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point2D, radius: float) -> List[Point2D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TK2], path: str) -> _TK2:
        """Load a 2D k-d tree from a file path."""
        ...


class KdTree3D:
    """A k-d tree for 3D points supporting NN and range queries.

    Uses Euclidean distance for searches.
    """

    def __init__(self) -> None:
        """Create an empty 3D k-d tree."""
        ...

    def insert(self, point: Point3D) -> None:
        """Insert a point; raises ValueError on invalid input."""
        ...

    def insert_bulk(self, points: List[Point3D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point3D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point3D, k: int) -> List[Point3D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point3D, radius: float) -> List[Point3D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TK3], path: str) -> _TK3:
        """Load a 3D k-d tree from a file path."""
        ...


class RTree2D:
    """An R-tree spatial index for 2D points.

    Balanced hierarchical index optimized for rectangle queries.
    """

    def __init__(self, max_entries: int) -> None:
        """Create an R-tree.

        Args:
            max_entries: Maximum entries per node (branching factor).
        """
        ...

    def insert(self, point: Point2D) -> None:
        """Insert a point into the index."""
        ...

    def insert_bulk(self, points: List[Point2D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point2D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point2D, k: int) -> List[Point2D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point2D, radius: float) -> List[Point2D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TR2], path: str) -> _TR2:
        """Load an R-tree from a file path."""
        ...


class RTree3D:
    """An R-tree spatial index for 3D points.

    Balanced hierarchical index optimized for rectangle queries.
    """

    def __init__(self, max_entries: int) -> None:
        """Create an R-tree.

        Args:
            max_entries: Maximum entries per node (branching factor).
        """
        ...

    def insert(self, point: Point3D) -> None:
        """Insert a point into the index."""
        ...

    def insert_bulk(self, points: List[Point3D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point3D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point3D, k: int) -> List[Point3D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point3D, radius: float) -> List[Point3D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TR3], path: str) -> _TR3:
        """Load an R-tree from a file path."""
        ...


class RStarTree2D:
    """An R*-tree spatial index for 2D points.

    Uses improved split heuristics over R-tree for better performance.
    """

    def __init__(self, max_entries: int) -> None:
        """Create an R*-tree.

        Args:
            max_entries: Maximum entries per node (branching factor).
        """
        ...

    def insert(self, point: Point2D) -> None:
        """Insert a point into the index."""
        ...

    def insert_bulk(self, points: List[Point2D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point2D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point2D, k: int) -> List[Point2D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point2D, radius: float) -> List[Point2D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TRS2], path: str) -> _TRS2:
        """Load an R*-tree from a file path."""
        ...


class RStarTree3D:
    """An R*-tree spatial index for 3D points.

    Uses improved split heuristics over R-tree for better performance.
    """

    def __init__(self, max_entries: int) -> None:
        """Create an R*-tree.

        Args:
            max_entries: Maximum entries per node (branching factor).
        """
        ...

    def insert(self, point: Point3D) -> None:
        """Insert a point into the index."""
        ...

    def insert_bulk(self, points: List[Point3D]) -> None:
        """Insert many points efficiently."""
        ...

    def delete(self, point: Point3D) -> bool:
        """Delete a point.

        Returns:
            True if the point was found and removed.
        """
        ...

    def knn_search(self, point: Point3D, k: int) -> List[Point3D]:
        """Find k nearest neighbors to the query point."""
        ...

    def range_search(self, point: Point3D, radius: float) -> List[Point3D]:
        """Find all points within a radius of the query point."""
        ...

    def save(self, path: str) -> None:
        """Serialize and save the tree to a file path."""
        ...

    @classmethod
    def load(cls: Type[_TRS3], path: str) -> _TRS3:
        """Load an R*-tree from a file path."""
        ...

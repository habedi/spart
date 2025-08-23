mod helpers;

#[cfg(test)]
mod tests {
    use super::helpers::Anyhow;
    use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
    use spart::kd_tree::KdTree;
    use spart::octree::Octree;
    use spart::quadtree::Quadtree;
    use spart::r_star_tree::RStarTree;
    use spart::r_tree::RTree;

    #[test]
    fn test_quadtree_serialization() -> Anyhow {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut qt = Quadtree::new(&boundary, 4).unwrap();
        qt.insert(Point2D::new(10.0, 20.0, Some("point1".to_string())));
        qt.insert(Point2D::new(50.0, 50.0, Some("point2".to_string())));

        let encoded: Vec<u8> = bincode::serialize(&qt)?;
        let decoded: Quadtree<String> = bincode::deserialize(&encoded[..])?;

        assert_eq!(
            qt.knn_search::<spart::geometry::EuclideanDistance>(&Point2D::new(12.0, 22.0, None), 1),
            decoded.knn_search::<spart::geometry::EuclideanDistance>(
                &Point2D::new(12.0, 22.0, None),
                1
            )
        );
        Ok(())
    }

    #[test]
    fn test_octree_serialization() -> Anyhow {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut octree = Octree::new(&boundary, 4).unwrap();
        octree.insert(Point3D::new(10.0, 20.0, 30.0, Some("point1".to_string())));
        octree.insert(Point3D::new(50.0, 50.0, 50.0, Some("point2".to_string())));

        let encoded: Vec<u8> = bincode::serialize(&octree)?;
        let decoded: Octree<String> = bincode::deserialize(&encoded[..])?;

        assert_eq!(
            octree.knn_search::<spart::geometry::EuclideanDistance>(
                &Point3D::new(12.0, 22.0, 32.0, None),
                1
            ),
            decoded.knn_search::<spart::geometry::EuclideanDistance>(
                &Point3D::new(12.0, 22.0, 32.0, None),
                1
            )
        );
        Ok(())
    }

    #[test]
    fn test_kdtree_serialization() -> Anyhow {
        let mut tree: KdTree<Point2D<String>> = KdTree::new();
        tree.insert(Point2D::new(1.0, 2.0, Some("A".to_string())))
            .unwrap();
        tree.insert(Point2D::new(3.0, 4.0, Some("B".to_string())))
            .unwrap();

        let encoded: Vec<u8> = bincode::serialize(&tree)?;
        let decoded: KdTree<Point2D<String>> = bincode::deserialize(&encoded[..])?;

        assert_eq!(
            tree.knn_search::<spart::geometry::EuclideanDistance>(&Point2D::new(2.0, 3.0, None), 1),
            decoded
                .knn_search::<spart::geometry::EuclideanDistance>(&Point2D::new(2.0, 3.0, None), 1)
        );
        Ok(())
    }

    #[test]
    fn test_rtree_serialization() -> Anyhow {
        let mut tree: RTree<Point2D<String>> = RTree::new(4).unwrap();
        tree.insert(Point2D::new(10.0, 20.0, Some("point1".to_string())));
        tree.insert(Point2D::new(50.0, 50.0, Some("point2".to_string())));

        let encoded: Vec<u8> = bincode::serialize(&tree)?;
        let decoded: RTree<Point2D<String>> = bincode::deserialize(&encoded[..])?;

        let query_rect = Rectangle {
            x: 5.0,
            y: 15.0,
            width: 10.0,
            height: 10.0,
        };
        assert_eq!(
            tree.range_search_bbox(&query_rect).len(),
            decoded.range_search_bbox(&query_rect).len()
        );
        Ok(())
    }

    #[test]
    fn test_rstartree_serialization() -> Anyhow {
        let mut tree: RStarTree<Point2D<String>> = RStarTree::new(4).unwrap();
        tree.insert(Point2D::new(10.0, 20.0, Some("point1".to_string())));
        tree.insert(Point2D::new(50.0, 50.0, Some("point2".to_string())));

        let encoded: Vec<u8> = bincode::serialize(&tree)?;
        let decoded: RStarTree<Point2D<String>> = bincode::deserialize(&encoded[..])?;

        let query_rect = Rectangle {
            x: 5.0,
            y: 15.0,
            width: 10.0,
            height: 10.0,
        };
        assert_eq!(
            tree.range_search_bbox(&query_rect).len(),
            decoded.range_search_bbox(&query_rect).len()
        );
        Ok(())
    }
}

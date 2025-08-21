from pyspart import RTree2D, RTree3D, Point2D, Point3D


def main():
    # --- 2D RTree Example ---
    print("--- 2D RTree Example ---")
    tree2d = RTree2D(4)

    # Insert some points
    tree2d.insert(Point2D(10.0, 20.0, 1))
    tree2d.insert(Point2D(80.0, 30.0, 2))
    tree2d.insert(Point2D(45.0, 70.0, 3))

    # Query the tree for points within a radius
    # Note: RTree in pyspart currently only supports range_search, not knn_search
    query_point_2d = Point2D(12.0, 22.0, None)
    results_2d = tree2d.range_search(query_point_2d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.data) for p in results_2d]}")

    # --- 3D RTree Example ---
    print("\n--- 3D RTree Example ---")
    tree3d = RTree3D(4)

    # Insert some points
    tree3d.insert(Point3D(10.0, 20.0, 30.0, 1))
    tree3d.insert(Point3D(80.0, 30.0, 40.0, 2))
    tree3d.insert(Point3D(45.0, 70.0, 50.0, 3))

    # Query the tree for points within a radius
    query_point_3d = Point3D(12.0, 22.0, 32.0, None)
    results_3d = tree3d.range_search(query_point_3d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.z, p.data) for p in results_3d]}")


if __name__ == "__main__":
    main()

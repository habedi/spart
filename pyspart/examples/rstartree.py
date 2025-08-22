from pyspart import RStarTree2D, RStarTree3D, Point2D, Point3D


def main():
    # --- 2D RStarTree Example ---
    print("--- 2D RStarTree Example ---")
    tree2d = RStarTree2D(4)

    # Insert some points
    tree2d.insert(Point2D(10.0, 20.0, 1))
    tree2d.insert(Point2D(80.0, 30.0, 2))
    tree2d.insert(Point2D(45.0, 70.0, 3))

    # KNN Search
    query_point_2d_knn = Point2D(12.0, 22.0, None)
    results_2d_knn = tree2d.knn_search(query_point_2d_knn, 1)
    print(f"1-NN search results: {[(p.x, p.y, p.data) for p in results_2d_knn]}")

    # Query the tree for points within a radius
    query_point_2d = Point2D(12.0, 22.0, None)
    results_2d = tree2d.range_search(query_point_2d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.data) for p in results_2d]}")

    # --- 3D RStarTree Example ---
    print("\n--- 3D RStarTree Example ---")
    tree3d = RStarTree3D(4)

    # Insert some points
    tree3d.insert(Point3D(10.0, 20.0, 30.0, 1))
    tree3d.insert(Point3D(80.0, 30.0, 40.0, 2))
    tree3d.insert(Point3D(45.0, 70.0, 50.0, 3))

    # KNN Search
    query_point_3d_knn = Point3D(12.0, 22.0, 32.0, None)
    results_3d_knn = tree3d.knn_search(query_point_3d_knn, 1)
    print(f"1-NN search results: {[(p.x, p.y, p.z, p.data) for p in results_3d_knn]}")

    # Query the tree for points within a radius
    query_point_3d = Point3D(12.0, 22.0, 32.0, None)
    results_3d = tree3d.range_search(query_point_3d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.z, p.data) for p in results_3d]}")


if __name__ == "__main__":
    main()

import os

from pyspart import Point2D, Point3D, RStarTree2D, RStarTree3D


def main():
    # --- 2D RStarTree Example ---
    print("--- 2D RStarTree Example ---")
    tree2d = RStarTree2D(4)

    # Insert some points
    points_to_insert_2d = [
        Point2D(10.0, 20.0, 1),
        Point2D(80.0, 30.0, 2),
        Point2D(45.0, 70.0, 3),
    ]
    tree2d.insert_bulk(points_to_insert_2d)

    # KNN Search
    query_point_2d_knn = Point2D(12.0, 22.0, None)
    results_2d_knn = tree2d.knn_search(query_point_2d_knn, 1)
    print(f"1-NN search results: {[(p.x, p.y, p.data) for p in results_2d_knn]}")

    # Query the tree for points within a radius
    query_point_2d = Point2D(12.0, 22.0, None)
    results_2d = tree2d.range_search(query_point_2d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.data) for p in results_2d]}")

    # Save and load the 2D tree
    path2d = "rstartree2d.spart"
    tree2d.save(path2d)
    loaded_tree2d = RStarTree2D.load(path2d)
    loaded_results_2d = loaded_tree2d.knn_search(query_point_2d_knn, 1)
    print(
        f"1-NN search results from loaded 2D tree: {[(p.x, p.y, p.data) for p in loaded_results_2d]}"
    )
    os.remove(path2d)

    # --- 3D RStarTree Example ---
    print("\n--- 3D RStarTree Example ---")
    tree3d = RStarTree3D(4)

    # Insert some points
    points_to_insert_3d = [
        Point3D(10.0, 20.0, 30.0, 1),
        Point3D(80.0, 30.0, 40.0, 2),
        Point3D(45.0, 70.0, 50.0, 3),
    ]
    tree3d.insert_bulk(points_to_insert_3d)

    # KNN Search
    query_point_3d_knn = Point3D(12.0, 22.0, 32.0, None)
    results_3d_knn = tree3d.knn_search(query_point_3d_knn, 1)
    print(f"1-NN search results: {[(p.x, p.y, p.z, p.data) for p in results_3d_knn]}")

    # Query the tree for points within a radius
    query_point_3d = Point3D(12.0, 22.0, 32.0, None)
    results_3d = tree3d.range_search(query_point_3d, 10.0)

    # Print the results
    print(f"Points within 10 units of query point: {[(p.x, p.y, p.z, p.data) for p in results_3d]}")

    # Save and load the 3D tree
    path3d = "rstartree3d.spart"
    tree3d.save(path3d)
    loaded_tree3d = RStarTree3D.load(path3d)
    loaded_results_3d = loaded_tree3d.knn_search(query_point_3d_knn, 1)
    print(
        f"1-NN search results from loaded 3D tree: {[(p.x, p.y, p.z, p.data) for p in loaded_results_3d]}"
    )
    os.remove(path3d)


if __name__ == "__main__":
    main()

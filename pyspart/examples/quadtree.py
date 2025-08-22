import os

from pyspart import Point2D, Quadtree


def main():
    # Create a new quadtree with a bounding box that spans from (0, 0) to (100, 100)
    boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
    quadtree = Quadtree(boundary, 4)

    # Insert some points into the quadtree
    points_to_insert = [
        Point2D(10.0, 20.0, 1),
        Point2D(80.0, 30.0, 2),
        Point2D(45.0, 70.0, 3),
    ]
    quadtree.insert_bulk(points_to_insert)

    # Query the quadtree for the 2 nearest neighbors to a point
    query_point = Point2D(12.0, 22.0, None)
    results = quadtree.knn_search(query_point, 2)

    # Print the results
    print(f"2 nearest neighbors to query point: {[(p.x, p.y, p.data) for p in results]}")

    # Save the quadtree to a file
    path = "quadtree.spart"
    quadtree.save(path)

    # Load the quadtree from the file
    loaded_quadtree = Quadtree.load(path)

    # Query the loaded quadtree
    loaded_results = loaded_quadtree.knn_search(query_point, 2)
    print(
        f"2 nearest neighbors to query point from loaded quadtree: {[(p.x, p.y, p.data) for p in loaded_results]}"
    )

    # Clean up the file
    os.remove(path)


if __name__ == "__main__":
    main()

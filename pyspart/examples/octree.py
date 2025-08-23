import os

from pyspart import Octree, Point3D


def main():
    # Create a new octree with a bounding box that spans from (0, 0, 0) to (100, 100, 100)
    boundary = {
        "x": 0.0,
        "y": 0.0,
        "z": 0.0,
        "width": 100.0,
        "height": 100.0,
        "depth": 100.0,
    }
    octree = Octree(boundary, 4)

    # Insert some points into the octree
    points_to_insert = [
        Point3D(10.0, 20.0, 30.0, 1),
        Point3D(80.0, 30.0, 40.0, 2),
        Point3D(45.0, 70.0, 50.0, 3),
    ]
    octree.insert_bulk(points_to_insert)

    # Query the octree for the 2 nearest neighbors to a point
    query_point = Point3D(12.0, 22.0, 32.0, None)
    results = octree.knn_search(query_point, 2)

    # Print the results
    print(f"2 nearest neighbors to query point: {[(p.x, p.y, p.z, p.data) for p in results]}")

    # Save the octree to a file
    path = "octree.spart"
    octree.save(path)

    # Load the octree from the file
    loaded_octree = Octree.load(path)

    # Query the loaded octree
    loaded_results = loaded_octree.knn_search(query_point, 2)
    print(
        f"2 nearest neighbors to query point from loaded octree: {[(p.x, p.y, p.z, p.data) for p in loaded_results]}"
    )

    # Clean up the file
    os.remove(path)


if __name__ == "__main__":
    main()

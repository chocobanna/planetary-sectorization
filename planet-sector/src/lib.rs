// lib.rs (for reference only)

use nalgebra::Vector3;

pub fn random_points_on_sphere(n: usize) -> Vec<Vector3<f64>> { /* ... */ }

pub fn spherical_voronoi(
    points: &[Vector3<f64>]
) -> Vec<Vec<Vector3<f64>>> { /* ... */ }

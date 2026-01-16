use nalgebra::Vector3;
use planet_sector::{random_points_on_sphere, spherical_voronoi};

fn main() {
    // Configuration
    let num_sites = 32;

    println!("Generating {} sites on the sphere…", num_sites);

    // Generate seed points
    let sites: Vec<Vector3<f64>> = random_points_on_sphere(num_sites);

    println!("Computing spherical Voronoi diagram…");

    // Compute Voronoi regions
    let regions = spherical_voronoi(&sites);

    println!("Done.");
    println!("Number of regions: {}", regions.len());

    // Dump some output so this binary actually *does* something
    for (i, region) in regions.iter().enumerate() {
        println!(
            "Region {:02}: {} vertices",
            i,
            region.len()
        );
    }
}

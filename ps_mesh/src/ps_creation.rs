use nalgebra::Point3;
type Point3D = nalgebra::Point3<f64>;
use super::point_mesh::PointsMesh;

// Stub function for creating a cube
pub fn create_cube(side: f32, step: f32, mesh : &mut PointsMesh) {
    mesh.points.clear();
    println!("Creating a cube with side {} and step {}", side, step);
    let side_elements = (side / step) as usize;
    
    // Adding points with a triple cycle.
    let start_corner_dist = side / 2. * step;
    for x_coord in 0..side_elements {
        for y_coord in 0..side_elements {
            for z_coord in 0..side_elements {
                mesh.points.push(Point3::new(
                    (x_coord as f32 * step - start_corner_dist) as f64, 
                    (y_coord as f32 * step - start_corner_dist) as f64,
                    (z_coord as f32 * step - start_corner_dist) as f64)
                )
            }
        }
    }
}

pub fn create_sphere(radius: f32, step: f32, mesh : &mut PointsMesh) {
    mesh.points.clear();
    println!("Creating a circle with radius {} and step {}", radius, step);
    let side_elements = (radius * 2. / step) as usize;
    
    // Adding points with a triple cycle.
    // Placing the points in a cube, adding only the ones within the radius.
    let start_corner_dist = radius * step;
    let center_point: Point3D = Point3::new(
        radius as f64,
        radius as f64,
        radius as f64,);
    for x_coord in 0..side_elements {
        for y_coord in 0..side_elements {
            for z_coord in 0..side_elements {
                let curr_point = Point3::new(
                    (x_coord as f32 * step - start_corner_dist) as f64, 
                    (y_coord as f32 * step - start_corner_dist) as f64,
                    (z_coord as f32 * step - start_corner_dist) as f64);
                if nalgebra::distance(&center_point, &curr_point) > radius as f64 {
                    continue;
                }
                mesh.points.push(curr_point);
            }
        }
    }
}

// Stub function for corroding the point cloud
pub async fn corrode(iterations: usize) {
    println!("Corroding point cloud with {} iterations", iterations);
    // Actual implementation of corrosion goes here
}

// Stub function for relaxing the point cloud
pub async fn relax(iterations: usize) {
    println!("Relaxing point cloud with {} iterations", iterations);
    // Actual implementation of relaxation goes here
}











// // OLD UNUSED
// pub fn create_random_cube(&mut self, num_points: usize, volume: (f64, f64, f64), breaking_range: (f64, f64)) {
//     let mut rng = rand::thread_rng();
//     self.points.clear();

//     for _ in 0..num_points {
//         self.points.push(Point3D::new(
//             rng.gen_range(-volume.0..volume.0),
//             rng.gen_range(-volume.1..volume.1),
//             rng.gen_range(-volume.2..volume.2),
//             rng.gen_range(breaking_range.0..breaking_range.1),
//         ));
//     }
// }
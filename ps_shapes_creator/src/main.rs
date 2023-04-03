use ps_data_layer::PointCloud;
use nalgebra::Point3;
use std::path::Path;



fn main() {
    let point_cloud = create_cube(50, 0.01).unwrap();
    point_cloud.write_to_file(Path::new("..\\points.bin"));
}


fn create_cube (side : usize, step : f32) -> Option<PointCloud> {
    
    // Adding points with a triple cycle.
    let mut cube_vector : Vec<Point3<f32>> = Vec::with_capacity(side*side*side);
    let start_corner_dist = side as f32/ 2. * step;
    for x_coord in 0..side {
        for y_coord in 0..side {
            for z_coord in 0..side {
                cube_vector.push(Point3::new(
                    x_coord as f32 * step - start_corner_dist, 
                    y_coord as f32 * step - start_corner_dist,
                    z_coord as f32 * step - start_corner_dist)
                )
            }
        }
    }

    Some(PointCloud::new(cube_vector))
}
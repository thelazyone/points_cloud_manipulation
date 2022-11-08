use kiss3d::nalgebra::{Point3};

// Fills a cubic Space.
pub fn fill_space_cube(i_cube_side : f32, i_linear_density : f32) -> Vec<(Point3<f32>, Point3<f32>)>{

    // Input checks:
    // TODO

    let mut points_cloud = Vec::new();

    // Creating coordinates in space, with grid
    let grid_steps = (i_cube_side / i_linear_density) as i32;
    for x_pos in (0..grid_steps).map(|x| x as f32 * i_linear_density - i_cube_side / 2.) {
        for y_pos in (0..grid_steps).map(|x| x as f32 * i_linear_density - i_cube_side / 2.) {
            for z_pos in (0..grid_steps).map(|x| x as f32 * i_linear_density - i_cube_side / 2.) {
                points_cloud.push((Point3::new(x_pos, y_pos, z_pos), Point3::new(1., 1., 1.)));
            }
        }
    } 
    points_cloud    
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

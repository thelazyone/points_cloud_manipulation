use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Translation3};



pub fn append_cloud_to_node(i_coordinates: Vec<Vector3<f64>>, i_node: &mut SceneNode){
    for coord_elem in i_coordinates.iter() {
        i_node
        .add_cube(0.01, 0.01, 0.01)
        .append_translation(&Translation3::new(
            coord_elem.x as f32, 
            coord_elem.y as f32, 
            coord_elem.z as f32));
    }

    println!("added {} points to the node.", i_coordinates.len())
}
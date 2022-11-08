use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Point3};

use crate::point_cloud_renderer as pcr;


pub fn append_cloud_to_renderer(
    i_points: Vec<(Point3<f32>, Point3<f32>)>, 
    i_object: &mut pcr::point_cloud_renderer){

    for point_elem in i_points.iter() {
        i_object
        .push(point_elem.0, point_elem.1);
    }

    println!("added {} points to the node.", i_points.len());
}
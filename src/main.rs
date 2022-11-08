extern crate kiss3d;
//extern crate kiss3d::nalgebra as na;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{UnitQuaternion, Vector3, Translation3};

mod utils;
use utils::general_utils::fill_space_cube;
use utils::mesh_utils::append_cloud_to_node;



struct AppState {
    c: SceneNode,
    rot: UnitQuaternion<f32>,
}

impl State for AppState {
    fn step(&mut self, _: &mut Window) {
        self.c.prepend_to_local_rotation(&self.rot)
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: wasm example");
    let mut c = window
        .add_cube(0.2, 0.01, 0.2);

    c.append_translation(&Translation3::new(0.0, -0.01, -0.0));
    c.set_color(1.0, 0.0, 0.0);


    let points_cloud = fill_space_cube(0.2, 0.01);
    append_cloud_to_node(points_cloud, &mut c);

    // c.add_cube(0.05, 0.05, 0.05).append_translation(&Translation3::new(0.0, -0.05, -0.2));

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let state = AppState { c, rot };

    window.render_loop(state)
}
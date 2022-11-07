use rand::{thread_rng};
extern crate kiss3d;

use kiss3d::nalgebra::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;


// fn gen_in_space() -> Vector3<f64>{
//     let mut rng = thread_rng();

//     Vector3::<f64> { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: rng.gen_range(-1.0..1.0)}
// }


fn main() {

    // First tests - creating a thick point cloud, relaxing it in some way and apply a delaunay mesh

    // let points = vec![
    //     gen_in_space(),
    //     gen_in_space(),
    //     gen_in_space(),
    // ];

    let mut window = Window::new("Kiss3d: cube");
    let mut c      = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }

    println!("Hello, world!");
}

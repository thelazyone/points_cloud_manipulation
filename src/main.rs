// Starting from the code found in https://github.com/sebcrozet/kiss3d/blob/master/examples/persistent_point_cloud.rs

extern crate kiss3d;
extern crate rand;

mod utils;
use utils::general_utils::fill_space_cube;
use utils::point_cloud_renderer;
use utils::mesh_utils;

use kiss3d::camera::Camera;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use kiss3d::nalgebra::{Point2, Point3};


// Custom renderers are used to allow rendering objects that are not necessarily
// represented as meshes. In this example, we will render a large, growing, point cloud
// with a color associated to each point.

// Writing a custom renderer requires the main loop to be
// handled by the `State` trait instead of a `while window.render()`
// like other examples.

struct AppState {
    point_cloud_renderer: point_cloud_renderer::point_cloud_renderer,
}

impl State for AppState {
    // Return the custom renderer that will be called at each
    // render loop.
    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (None, None, Some(&mut self.point_cloud_renderer), None)
    }

    fn step(&mut self, window: &mut Window) {
        let num_points_text = format!(
            "Number of points: {}",
            self.point_cloud_renderer.num_points()
        );
        window.draw_text(
            &num_points_text,
            &Point2::new(0.0, 20.0),
            60.0,
            &Font::default(),
            &Point3::new(1.0, 1.0, 1.0),
        );
    }
}

fn main() {
    let window = Window::new("Points cloud visualization");
    let mut app = AppState {
        point_cloud_renderer: point_cloud_renderer::point_cloud_renderer::new(4.0),
    };

    // Adding the points to the renderer:
    let point_cloud = fill_space_cube(0.5, 0.0025);
    mesh_utils::append_cloud_to_renderer(point_cloud, &mut app.point_cloud_renderer);

    window.render_loop(app)
}


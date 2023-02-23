
extern crate kiss3d;
extern crate rand;

mod utils;
use utils::general_utils::fill_space_cube;
use utils::point_cloud_renderer;
use utils::mesh_utils;
use utils::app_state;

use kiss3d::window::{Window};

fn main() {
    let window = Window::new("Points cloud visualization");
    let mut app = app_state::AppState {
        point_cloud_manager: point_cloud_renderer::PointsCloudManager::new(4.0),
    };

    // Adding the points to the renderer:
    let point_cloud = fill_space_cube(0.5, 0.0025);
    mesh_utils::append_cloud_to_renderer(point_cloud, &mut app.point_cloud_manager);

    window.render_loop(app)
}


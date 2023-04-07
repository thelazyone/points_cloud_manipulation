use ps_data_layer::PointCloud;

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use kiss3d::{light::Light};
use kiss3d::window::Window;
use kiss3d::nalgebra::Point3;

// Polling on the target file
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};


fn main() {
    // Set up Kiss3D window
    let mut window = Window::new("3D Visualizer");

    // Set up the 3D rendering components (camera, lights, etc.)
    window.set_light(Light::StickToCamera);
    let mut point_cloud = PointCloud::new(Vec::<Point3<f32>>::new());
    let target_path = point_cloud.get_standard_file();
    let target_path = Path::new(target_path);

    // Rendering once before starting the loop:
    point_cloud.read_from_file(target_path.clone());
    draw_points (&point_cloud, &mut window);

    // Set up the notify file watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(target_path, RecursiveMode::NonRecursive)
        .unwrap();

    while window.render() {
            // Check for file changes
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(path))
            | Ok(DebouncedEvent::Write(path))
            | Ok(DebouncedEvent::Rename(_, path)) => {
                
                if ps_data_layer::are_paths_same(
                    &path,
                    &Path::new(point_cloud.get_standard_file())) {

                    // Load the point cloud data from the binary file
                    point_cloud.read_from_file(&path);
                }
            }
            _ => {}
        }

        // Render the point cloud
        draw_points (&point_cloud, &mut window);
    }
}


fn draw_points (i_cloud : &PointCloud, i_window : &mut Window) {
    
    // WARNING: nalgebra and kiss3D::nalgebra are separate objects, warning!""
    for point in &i_cloud.points {
        let color = kiss3d::nalgebra::geometry::Point3::<f32>::new(1.0, 1.0, 1.0);

        // Temporarly converting nalgebra into kiss3d::nalgebra because somehow it's in conflict.
        let position = kiss3d::nalgebra::Point3::<f32>::new(point.x, point.y, point.z);
        i_window.draw_point(&position, &color);
    }
}
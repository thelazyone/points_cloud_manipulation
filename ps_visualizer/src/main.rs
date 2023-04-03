use ps_data_layer::PointCloud;

use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use kiss3d::light::Light;
use kiss3d::window::Window;
use nalgebra::Point3;

// Polling on the target file
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

const POINTS_FILE: &str = "C:\\Projects\\points_cloud_manipulation\\points.bin";


fn main() {
    // Set up Kiss3D window
    let mut window = Window::new("3D Visualizer");

    // Set up the 3D rendering components (camera, lights, etc.)
    window.set_light(Light::StickToCamera);

    let mut point_cloud = PointCloud::new(Vec::<Point3<f32>>::new());

    // Set up the notify file watcher
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    watcher
        .watch(Path::new(POINTS_FILE), RecursiveMode::NonRecursive)
        .unwrap();

    while window.render() {
            // Check for file changes
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(path))
            | Ok(DebouncedEvent::Write(path))
            | Ok(DebouncedEvent::Rename(_, path)) => {
                if path == Path::new(POINTS_FILE) {
                    // Load the point cloud data from the binary file
                    point_cloud.read_from_file(&path);
                }
            }
            _ => {}
        }

        // Render the point cloud
        // WARNING: nalgebra and kiss3D::nalgebra are separate objects, warning!""
        for point in &point_cloud.points {
            let color = kiss3d::nalgebra::geometry::Point3::<f32>::new(1.0, 1.0, 1.0);

            // Temporarly converting nalgebra into kiss3d::nalgebra because somehow it's in conflict.
            let position = kiss3d::nalgebra::Point3::<f32>::new(point.x, point.y, point.z);
            window.draw_point(&position, &color);
        }
    }
}
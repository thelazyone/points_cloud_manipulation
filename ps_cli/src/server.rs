use ps_mesh::PointsMesh;

// For the real-time visualization  
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;
use tokio::time::{Duration, Instant, interval_at};
use futures_util::SinkExt;


pub async fn handle_ws_connection(ws: warp::ws::WebSocket, point_cloud: Arc<Mutex<PointsMesh>>, clients: Arc<Mutex<Vec<Arc<Mutex<warp::ws::WebSocket>>>>>) {
    // Add the WebSocket to the list of connected clients
    {
        let mut clients = clients.lock().await;
        clients.push(Arc::new(Mutex::new(ws)));
    }
    let mut interval = interval_at(Instant::now(), Duration::from_millis(100)); // Update every 100ms, adjust as needed

    loop {
        interval.tick().await;

        let point_cloud_data = {
            let point_cloud = point_cloud.lock().await;
            points_mesh_to_json(&point_cloud)
        };

        let mut clients = clients.lock().await;
        for client in clients.iter() {
            let mut client = client.lock().await;

            if let Err(e) = client.send(warp::ws::Message::text(point_cloud_data.clone())).await {
                eprintln!("Error sending data to client: {:?}", e);
            }
        }
    }
}


pub fn points_mesh_to_json(points_mesh: &PointsMesh) -> String {
    json!(points_mesh.get_points_for_display()).to_string()
}
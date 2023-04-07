use ps_data_layer::PointCloud;
use ps_mesh::PointsMesh;
use ps_mesh::Point3D;
use kiss3d::nalgebra::Point3;
use std::path::Path;

// For the CLI
use clap::Parser;
use clap::Subcommand;
use rustyline::{error::ReadlineError, Editor};

// For the real-time visualization
use std::sync::Arc;
use warp::Filter;
use tokio::sync::Mutex;
use serde_json::json;
use tokio::time::{Duration, Instant, interval_at};
use futures_util::{SinkExt};


#[derive(Parser, Debug)]
#[command(name = "ps_cli")]
#[command(author = "Giacomo Pantalone")]
#[command(version = "1.0")]
#[command(about = "A CLI for quick point clouds manipulations")]
struct CliArguments {

    #[command(subcommand)]
    command: Option<CliCommand>,
}

// The commands structure: 
#[derive(Parser, Debug)]
enum CliCommand {
    #[command(subcommand)]
    Create(CreateCommand),

    // Without subcommands
    Corrode(CorrodeCommand),
    Relax(RelaxCommand),
}


#[derive(Subcommand, Debug)]   
enum CreateCommand {
    Cube {
        #[arg(long, default_value = "1")]
        side: f32,
    
        #[arg(long, default_value = "0.02")]
        step: f32,
    },
    Circle {
        #[arg(long, default_value = "2")]
        radius: f32,
    
        #[arg(long, default_value = "0.03")]
        step: f32,
    }
}


#[derive(Parser, Debug)]
struct CorrodeCommand {
    #[arg(long, default_value = "100")]
    iterations: usize,
}


#[derive(Parser, Debug)]
struct RelaxCommand {
    #[arg(long, default_value = "3")]
    iterations: usize,
}

#[tokio::main]
async fn main() {
    
    // shared space of point cloud
    let points_mesh = Arc::new(Mutex::new(PointsMesh::new()));

    // Start the HTTP server in a separate thread
    let points_mesh_clone = points_mesh.clone();
    let clients = Arc::new(Mutex::new(Vec::new()));
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
    
        rt.block_on(async move {
            run_server(points_mesh_clone, clients).await;
        });
    });
    //local_set.await;

    println!("Initializing the interactive CLI...");
    let mut rl = Editor::<()>::new();

    // Defining the rustyline interactive CLI 
    loop {
        let readline = rl.readline("ps-cli> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let cli_args = format!("ps_cli {}", line);
                let args = match CliArguments::try_parse_from(cli_args.split_whitespace()) {
                    Ok(args) => args,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        continue;
                    }
                };

                if let Some(command) = args.command {
                    match command {
                        CliCommand::Create(create_command) => 
                            match create_command {
                                CreateCommand::Cube { side, step } => {
                                    //create_cube(side, step, &mut points_mesh).await;
                                    let mut mesh = points_mesh.lock().await;
                                    mesh.points.clear();
                                    for point in 0..10 {
                                        mesh.points.push(Point3D::new(point as f64, point as f64, point as f64, 0.));
                                    }
                                    
                                },
                                CreateCommand::Circle { radius, step } => {
                                    //create_circle(radius, step, &mut points_mesh).await,
                                }
                            },
                        CliCommand::Corrode(corrode_command) => corrode(corrode_command.iterations),
                        CliCommand::Relax(relax_command) => relax(relax_command.iterations),
                    }
                }

            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

async fn run_server(point_cloud: Arc<Mutex<PointsMesh>>, clients: Arc<Mutex<Vec<Arc<Mutex<warp::ws::WebSocket>>>>>) {
    // Serve the static files
    let static_files = warp::fs::dir("./static");

    // WebSocket endpoint for sending point cloud updates
    let clients_clone = clients.clone();
    let point_cloud_clone = point_cloud.clone();
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let point_cloud = point_cloud_clone.clone();
            let clients = clients_clone.clone();
            ws.on_upgrade(move |socket| handle_ws_connection(socket, point_cloud, clients))
        });

    // Combine filters and start the server
    let routes = static_files.or(ws_route);
    println!("Serving on 127.0.0.1:3030...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


async fn handle_ws_connection(ws: warp::ws::WebSocket, point_cloud: Arc<Mutex<PointsMesh>>, clients: Arc<Mutex<Vec<Arc<Mutex<warp::ws::WebSocket>>>>>) {
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


fn points_mesh_to_json(points_mesh: &PointsMesh) -> String {
    json!(points_mesh.get_points_for_display()).to_string()
}


// TODO TBR!
// CREATION FUNCTIONS (to be moved in ps_mesh TODO)

// Stub function for creating a cube
async fn create_cube(side: f32, step: f32, mesh : &mut Arc<Mutex<PointsMesh>>) {
    println!("Creating a cube with side {} and step {}", side, step);
    let side_elements = (side / step) as usize;
    
    // Adding points with a triple cycle.
    let mut cube_vector : Vec<Point3<f32>> = Vec::with_capacity(side_elements*side_elements*side_elements);
    let start_corner_dist = side as f32/ 2. * step;
    for x_coord in 0..side_elements {
        for y_coord in 0..side_elements {
            for z_coord in 0..side_elements {
                cube_vector.push(Point3::new(
                    x_coord as f32 * step - start_corner_dist, 
                    y_coord as f32 * step - start_corner_dist,
                    z_coord as f32 * step - start_corner_dist)
                )
            }
        }
    }
    let point_cloud = PointCloud::new(cube_vector);

    // Preparing the mesh.
    let mut mesh = mesh.lock().await;
    mesh.points.clear();
    for point in point_cloud.points {
        mesh.points.push(Point3D::new(point.x as f64, point.y as f64, point.z as f64, 0.));
    }
    
    // println!("Writing a cube in {}...", point_cloud.get_standard_file());
    // point_cloud.write_to_file(Path::new(point_cloud.get_standard_file()));
    // println!("Done!");
}

// Stub function for creating a circle
async fn create_circle(radius: f32, step: f32, mesh : &mut Arc<Mutex<PointsMesh>>) {
    println!("Creating a circle with radius {} and step {}", radius, step);
    // Actual implementation of circle creation goes here
}

// Stub function for corroding the point cloud
fn corrode(iterations: usize) {
    println!("Corroding point cloud with {} iterations", iterations);
    // Actual implementation of corrosion goes here
}

// Stub function for relaxing the point cloud
fn relax(iterations: usize) {
    println!("Relaxing point cloud with {} iterations", iterations);
    // Actual implementation of relaxation goes here
}

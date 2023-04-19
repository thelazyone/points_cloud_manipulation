use ps_mesh::point_mesh::PointsMesh as PointsMesh;
use ps_mesh::ps_creation::*;

mod cli_arguments;
use cli_arguments::*;

// For the CLI
use clap::Parser;
use clap::Subcommand;
use rustyline::{error::ReadlineError, Editor};

// For the real-time visualization
use std::sync::Arc;
use warp::Filter;
use tokio::sync::Mutex;

// Local files
mod server;
use server::handle_ws_connection;

// Alias
type Point3D = nalgebra::Point3<f64>;

#[tokio::main]
async fn main() {
    
    // shared space of point cloud
    let points_mesh: Arc<Mutex<PointsMesh>> = Arc::new(Mutex::new(PointsMesh::new()));

    // Start the HTTP server in a separate thread
    let points_mesh_clone: Arc<Mutex<PointsMesh>> = points_mesh.clone();
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
                                    let mut mesh = points_mesh.lock().await;
                                    create_cube(side, step,  &mut *mesh);
                                },
                                CreateCommand::Sphere { radius, step } => {
                                    let mut mesh = points_mesh.lock().await;
                                    create_sphere(radius, step,  &mut *mesh);
                                }
                            },
                        CliCommand::Clear(_) => {
                            let mut mesh = points_mesh.lock().await;
                            mesh.points.clear();
                        }
                        CliCommand::Corrode(corrode_command) => continue,//corrode(corrode_command.iterations),
                        CliCommand::Relax(relax_command) => continue,//relax(relax_command.iterations),
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
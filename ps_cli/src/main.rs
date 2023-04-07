use ps_data_layer::PointCloud;
use kiss3d::nalgebra::Point3;
use std::path::Path;

// Point clouds manipulator CLI, following the steps of the tutorial 
// at https://rust-cli.github.io/book/tutorial/cli-args.html 

use clap::Parser;
use clap::Subcommand;
use rustyline::{error::ReadlineError, Editor};

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

fn main() {

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
                        CliCommand::Create(create_command) => match create_command {
                            CreateCommand::Cube { side, step } => create_cube(side, step),
                            CreateCommand::Circle { radius, step } => create_circle(radius, step),
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


// CREATION FUNCTIONS (to be moved elsewhere TODO)

// Stub function for creating a cube
fn create_cube(side: f32, step: f32) {
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
    
    println!("Writing a cube in {}...", point_cloud.get_standard_file());
    point_cloud.write_to_file(Path::new(point_cloud.get_standard_file()));
    println!("Done!");
}

// Stub function for creating a circle
fn create_circle(radius: f32, step: f32) {
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

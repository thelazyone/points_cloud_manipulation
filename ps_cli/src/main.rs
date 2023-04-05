use ps_data_layer::PointCloud;
use kiss3d::nalgebra::Point3;
use std::path::Path;



// Point clouds manipulator CLI, following the steps of the tutorial 
// at https://rust-cli.github.io/book/tutorial/cli-args.html 

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(name = "ps_cli")]
#[command(author = "Giacomo Pantalone")]
#[command(version = "1.0")]
#[command(about = "A CLI for quick point clouds manipulations", long_about = None)]
struct CliArguments {

    #[command(subcommand)]
    command: Option<Shapes>,

    // /// Sets a custom config file
    // #[arg(long, value_name = "SIDES")]
    // cube_sides: Option<u32>,
}

#[derive(Debug)]
#[derive(Subcommand)]   
enum Shapes {
    /// does testing things
    Cube {
        #[arg(long)]
        sides: u32,

        #[arg(long)]
        step: f32,
    },

    Sphere {
        #[arg(long)]
        radius: f32,

        #[arg(long)]
        step: f32,
    },
}

fn main() {

    // Retrieving the command line arguments
    let args = CliArguments::parse();

    // Depending on the command, matching:
    match &args.command {
        Some(Shapes::Cube { sides, step }) => {
            let point_cloud = create_cube(*sides as usize, *step as f32).unwrap();
            println!("Writing a cube in {}...", point_cloud.get_standard_file());
            point_cloud.write_to_file(Path::new(point_cloud.get_standard_file()));
            println!("Done!");
        }
        Some(Shapes::Sphere { radius, step }) => {
            // Do nothing for sphere
        }
        None => {}
    }

    println!("Hello, world!");
}







// To be moved elsewhere
fn create_cube (side : usize, step : f32) -> Option<PointCloud> {
    
    // Adding points with a triple cycle.
    let mut cube_vector : Vec<Point3<f32>> = Vec::with_capacity(side*side*side);
    let start_corner_dist = side as f32/ 2. * step;
    for x_coord in 0..side {
        for y_coord in 0..side {
            for z_coord in 0..side {
                cube_vector.push(Point3::new(
                    x_coord as f32 * step - start_corner_dist, 
                    y_coord as f32 * step - start_corner_dist,
                    z_coord as f32 * step - start_corner_dist)
                )
            }
        }
    }

    Some(PointCloud::new(cube_vector))
}
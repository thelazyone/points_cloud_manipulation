// Geometry
use kiss3d::nalgebra::Point3;

// Filesystem and I/O
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;
use std::io::Write;
use std::io::BufWriter;


// Local paths:
const POINTS_FILE: &str = "maps/points.bin";

// Points Cloud object
#[derive(Debug)]
pub struct PointCloud {
    pub points : Vec<Point3<f32>>,
}


impl PointCloud {
    pub fn new (i_points : Vec<Point3<f32>>) -> PointCloud {
        PointCloud {points: i_points,}
    }

    pub fn new_from_file(i_path : &Path) -> PointCloud{
        PointCloud::new(read_points_from_binary(i_path).unwrap())
    }

    pub fn get_standard_file(&self) -> & 'static str {
        POINTS_FILE
    }

    pub fn read_from_file(&mut self, i_path : &Path) -> Option<usize> {

        if let Ok(points) = read_points_from_binary(i_path) {
            self.points = points;
            return Some(self.points.len());
        }
        None
    }
    
    pub fn write_to_file(&self, i_path: &Path) -> Option<usize> {
        if let Ok(points) = write_points_to_binary(i_path, &self.points) {
            return Some(points)
        }
        None
    }

}


// Private functions 
fn read_points_from_binary(file_path: &Path) -> std::io::Result<Vec<Point3<f32>>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut points = Vec::new();
    let mut buffer = [0u8; 4 * 3]; // 3 f32 values, 4 bytes each

    // TODO rewrite this, it's ugly!
    while reader.read_exact(&mut buffer).is_ok() {
        let x = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let y = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let z = f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);

        points.push(Point3::new(x, y, z));
    }

    Ok(points)
}


fn write_points_to_binary(file_path: &Path, points: &Vec<Point3<f32>>) -> std::io::Result<usize> {
    println!("writing file {}", file_path.to_str().unwrap());

    // Creating the path if necessary:
    let prefix = file_path.parent().unwrap();
    println!("Creating folder {}", prefix.to_str().unwrap());
    std::fs::create_dir_all(prefix).unwrap();

    // Creating the file.
    let file = File::create(file_path);
    if file.is_err() {
        panic!("Cannot write file {}", file_path.as_os_str().to_str().unwrap())
    };
    
    let file = file?;
    
    println!("file created.");
    let mut writer = BufWriter::new(file);

    let mut points_counter = 0;
    for &point in points {
        writer.write_all(&point.x.to_le_bytes())?;
        writer.write_all(&point.y.to_le_bytes())?;
        writer.write_all(&point.z.to_le_bytes())?;
        points_counter += 1;
    }

    writer.flush()?;

    println!("Written {} points.", points_counter);

    Ok(points_counter)
}




// Utilities Functions:
pub fn are_paths_same (path_a: &Path, path_b: &Path) -> bool {
    std::fs::canonicalize(path_a).unwrap() == std::fs::canonicalize(path_b).unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_test() {
        let mut test_vec = Vec::<Point3<f32>>::new();
        test_vec.push(Point3::new(42.,0.,1.));
        test_vec.push(Point3::new(43.,1.,2.));
        test_vec.push(Point3::new(44.,2.,3.));

        let test_path = Path::new(".\\test.bin");
        
        // Writing
        assert_eq!(3, write_points_to_binary(&test_path, &test_vec).expect("Writing failed"), "Wrote the wrong amount of points");
        
        // Reading
        let read_vec = read_points_from_binary(&test_path).expect("Reading failed");
        assert_eq!(3, read_vec.len(), "Read the wrong amount of points");
        assert_eq!(test_vec, read_vec, "input output vector mismatch");
    }
}

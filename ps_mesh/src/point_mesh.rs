use std::collections::HashMap;
use std::collections::HashSet;
use nalgebra::Point3;
type Point3D = nalgebra::Point3<f64>;

pub enum BreakingPoint {
    Broken, 
    Intact(f64),
}

pub struct PointsMesh {
    pub points: Vec<Point3<f64>>,
    pub energies: Vec<f64>,
    pub breaking_point: Vec<BreakingPoint>,
    pub connections: HashMap<usize, Vec<usize>>,
}


impl PointsMesh {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            energies: Vec::new(),
            breaking_point: Vec::new(),
            connections: HashMap::new(),
        }
    }
    
    pub fn create_points(&mut self, points: Vec<Point3D>) {

    }


    pub fn establish_connections(&mut self, radius: f64) {
        self.connections.clear();

        for i in 0..self.points.len() {
            for j in (i + 1)..self.points.len() {
                let distance = nalgebra::distance(&self.points[i], &self.points[j]);

                if distance <= radius {
                    self.connections.entry(i).or_insert_with(Vec::new).push(j);
                    self.connections.entry(j).or_insert_with(Vec::new).push(i);
                }
            }
        }
    }

    pub fn remove_unconnected_points(&mut self) {
        let unconnected_points: HashSet<usize> = self.points
            .iter()
            .enumerate()
            .filter(|(index, _point)| !self.connections.contains_key(index))
            .map(|(index, _point)| index)
            .collect();
    
        let mut new_points = Vec::new();
        let mut index_mapping = HashMap::new();
        let mut new_index = 0;
    
        for (old_index, point) in self.points.iter().enumerate() {
            if !unconnected_points.contains(&old_index) {
                new_points.push(point.clone());
                index_mapping.insert(old_index, new_index);
                new_index += 1;
            }
        }
    
        self.points = new_points;
    
        let mut new_connections = HashMap::new();
    
        for (old_key, indices) in self.connections.iter() {
            let new_key = *index_mapping.get(old_key).unwrap();
    
            let new_indices: Vec<usize> = indices
                .iter()
                .map(|old_index| *index_mapping.get(old_index).unwrap())
                .collect();
    
            new_connections.insert(new_key, new_indices);
        }
    
        self.connections = new_connections;
    }

    pub fn average_connections_per_point(&self) -> f64 {
        let total_connections: usize = self.connections.values().map(|v| v.len()).sum();
        total_connections as f64 / self.points.len() as f64
    }

    pub fn get_statistics(&self) -> (f64, usize) {
        let average_connections = self.average_connections_per_point();
        let unconnected_points = self.points.len() - self.connections.len();

        (average_connections, unconnected_points)
    }

    
    pub fn relaxation_step(&mut self, relaxation_factor: f64) { 
        let mut new_positions = Vec::with_capacity(self.points.len());
    
        for (index, _) in self.points.iter().enumerate() {
            let mut count = 0;
            let mut sum = Point3D::new(0.0, 0.0, 0.0);
    
            for neighbor_index in self.connections.get(&index).unwrap_or(&vec![]).iter() {
                let neighbor = &self.points[*neighbor_index];
                sum.x += neighbor.x;
                sum.y += neighbor.y;
                sum.z += neighbor.z;
                count += 1;
            }
    
            if count > 0 {
                let mut current_point = self.points[index].clone();
                current_point.x = current_point.x * (1.0 - relaxation_factor) + (sum.x / count as f64) * relaxation_factor;
                current_point.y = current_point.y * (1.0 - relaxation_factor) + (sum.y / count as f64) * relaxation_factor;
                current_point.z = current_point.z * (1.0 - relaxation_factor) + (sum.z / count as f64) * relaxation_factor;
                new_positions.push(current_point);
            } else {
                new_positions.push(self.points[index].clone());
            }
        }
    
        // After finding the new positions, replacing the old positions of the points.
        self.points = new_positions;
    }


    pub fn get_points_for_display(&self) -> Vec<Vec<f64>> {
        self.points.iter().map(|point| vec![point.x, point.y, point.z]).collect::<Vec<_>>()
    }


    pub fn get_point_connections(&self, index: usize) -> Option<&Vec<usize>> {
        self.connections.get(&index)
    }
}

# 3D Point Cloud Toolkit
This library contains various tools I'm using to play around with point clouds and voxels. 
The long-term goal is to create useful tools to manipulate meshes to represent growth and erosion for rocks, trees, concrete.

The 3D visualization is implemented in [Kiss3D](https://github.com/sebcrozet/kiss3d) which seemed to be the quickest way to just show some points in a window. It also implemented a GPU-accelerated visualization which could be implemented to better handle, say, 100 million points or more.

## Usage
The toolkit is in a very early stage. Right now a basic visualizer is implemented, as well as a data layer that defines the internal files format.
On one terminal start the visualizer with <code>cargo run --release -p ps_visualizer</code>. The visualizer keeps polling the ./maps/points.bin file. 
Currently the ps_shapes_creator is an absolutely basic test project which fills a small cube of points and can be run with <code>cargo run --release -p ps_shapes_creator</code>.

## Future Steps
Basically _everything_. Realistically, it would be wise to focus either on voxels or floating points.
The three lists below are somehow orthogonal in terms of development.

For Floating Points:
- [ ] Implement the 3D Delunay algorithm
- [ ] Implement a STL-To-Points tool
- [ ] Extract STL files 

For Voxels:
- [ ] Implement the basic visualizator
- [ ] Implement the Marching Cubes algorithm
- [ ] Merge vertexes and extract STL files

For Simulations (after deciding which road to take of the above):
- [ ] Implement a basic erosion system (smoothing)
- [ ] Implement a ray-tracing erosion system
- [ ] Implement a Fracture system which looks a bit better than Foronoi fracture

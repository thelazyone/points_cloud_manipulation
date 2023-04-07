const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Create an empty BufferGeometry
const geometry = new THREE.BufferGeometry();

// Create a PointsMaterial
const material = new THREE.PointsMaterial({ color: 0xffffff, size: 0.1 });

// Create a THREE.Points object
const pointCloud = new THREE.Points(geometry, material);

// Add the pointCloud to the scene
scene.add(pointCloud);

/// Websocket stuff:
// Connect to the WebSocket server
const socket = new WebSocket('ws://localhost:3030/ws');

// Set up the WebSocket event listeners
socket.addEventListener('open', (event) => {
    console.log('WebSocket connection opened:', event);

    // Request the point cloud data from the server
    socket.send('request_data');
});

socket.addEventListener('message', (event) => {
    console.log('WebSocket message received:', event);

    // Parse the received data and update the point cloud visualization
    const points_mesh = JSON.parse(event.data);
    update_point_cloud(points_mesh);
});

socket.addEventListener('close', (event) => {
    console.log('WebSocket connection closed:', event);
});

socket.addEventListener('error', (event) => {
    console.log('WebSocket error:', event);
});

function update_point_cloud(points_data) {
    // Create a Float32Array to store the vertices
    const vertices = new Float32Array(points_data.length * 3);

    // Fill the vertices array with the data from points_data
    for (let i = 0; i < points_data.length; i++) {
        const [x, y, z] = points_data[i];
        vertices[i * 3] = x;
        vertices[i * 3 + 1] = y;
        vertices[i * 3 + 2] = z;
    }

    // Update the BufferGeometry with the new vertices
    pointCloud.geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    pointCloud.geometry.attributes.position.needsUpdate = true;
}

const animate = function () {
    requestAnimationFrame(animate);

    // Add any animations or updates to the scene here

    renderer.render(scene, camera);
};

animate();
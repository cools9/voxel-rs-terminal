# Voxel Engine
 
A voxel engine built in Rust using [raylib-rs](https://github.com/raylib-rs/raylib-rs).
 
The moment you first open it, you might see nothing — so slowly move your mouse to the right, and you'll spot a big, weird-looking cuboid. That's the project.
 
## What is this?
 
This started as an attempt at a Minecraft clone. For every hour I spent coding, it got harder and harder, so the goal shifted: instead of building a full Minecraft clone, this is now about rendering as many triangles as possible without my GPU exploding.
 
## Features (so far)
 
- Chunked world generation (`FxHashMap<ChunkPos, Chunk>`, 16³ voxels per chunk)
- Correct negative-coordinate chunk mapping via `div_euclid`/`rem_euclid`
- Neighbor-based face culling — only visible faces get meshed
- Static GPU-uploaded meshes per chunk (moved off immediate-mode `draw_triangle3D` for a major performance win)
- Free-look camera with mouse look (yaw/pitch) and WASD movement relative to look direction
- A crosshair, because it wouldn't be an FPS-shaped project without one
## Controls
 
| Key | Action |
|-----|--------|
| `W` / `A` / `S` / `D` | Move |
| `Space` | Move up |
| `Left Shift` | Move down |
| Mouse | Look around |
 
## Running it on your computer
 
```bash
cargo install voxel-rs
```
 
## Status
 
Actively in progress — terrain generation, raycasting/block interaction, and rendering optimizations are all being worked on. Expect weird cuboids.
 
## Acknowledgements
 
Architecture and implementation decisions were informed by studying [minecrab](https://github.com/jabacat/minecrab), an open-source Rust/raylib voxel game.

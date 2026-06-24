use raylib::prelude::*;
//use raylib::consts::CameraMode;
use rustc_hash::FxHashMap;

use crate::VoxelType::NoName;

struct Player {
    x: i64,
    y: i64,
    z: i64,
}
enum movement {
    FRONT,
    BACK,
    LEFT,
    RIGHT,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Voxel {
    kind: VoxelType,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct VoxelPos {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VoxelType {
    NoName,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ChunkPos {
    x: i64,
    y: i64,
    z: i64,
}

const CHUNK_SIZE: i64 = 16;

type Chunk = FxHashMap<VoxelPos, Voxel>;
type World = FxHashMap<ChunkPos, Chunk>;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("demo")
        .fullscreen()
        .vsync()
        .build();
    let world_size_chunks: i64 = 4;
    let mut player = Player { x: 0, y: 0, z: 0 };
    let mut camera = Camera3D::perspective(
        Vector3::new(player.x as f32, player.y as f32, player.z as f32),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let world = generate_world(world_size_chunks);
    let models = build_world_meshes(&world, &thread, &mut rl);

    let mut direction = movement::FRONT;
    while !rl.window_should_close() {
        rl.disable_cursor(); // call this once outside the loop ideally, not every frame

        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_UP) | Some(KeyboardKey::KEY_W) => {
                direction = movement::FRONT;
            }
            Some(KeyboardKey::KEY_DOWN) | Some(KeyboardKey::KEY_S) => {
                direction = movement::BACK;
            }
            Some(KeyboardKey::KEY_LEFT) | Some(KeyboardKey::KEY_A) => {
                direction = movement::LEFT;
            }
            Some(KeyboardKey::KEY_RIGHT) | Some(KeyboardKey::KEY_D) => {
                direction = movement::RIGHT;
            }
            _ => {}
        }

        match direction {
            movement::FRONT => player.x += 5,
            movement::BACK => player.x -= 5,
            movement::LEFT => player.z -= 5,
            movement::RIGHT => player.z += 5,
        }
        camera.position = Vector3::new(player.x as f32, player.y as f32, player.z as f32);
        camera.target = Vector3::new(player.x as f32, player.y as f32, player.z as f32 + 1.0);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);

        {
            let mut c = d.begin_mode3D(camera);
            for (chunk_pos, model) in &models {
                let offset = Vector3::new(
                    (chunk_pos.x * CHUNK_SIZE) as f32,
                    (chunk_pos.y * CHUNK_SIZE) as f32,
                    (chunk_pos.z * CHUNK_SIZE) as f32,
                );
                c.draw_model(model, offset, 1.0, Color::WHITE);
            }
        }
    }
}

fn voxel_to_chunk_pos(pos: VoxelPos) -> ChunkPos {
    ChunkPos {
        x: pos.x.div_euclid(CHUNK_SIZE),
        y: pos.y.div_euclid(CHUNK_SIZE),
        z: pos.z.div_euclid(CHUNK_SIZE),
    }
}

fn voxel_to_local_pos(pos: VoxelPos) -> VoxelPos {
    VoxelPos {
        x: pos.x.rem_euclid(CHUNK_SIZE),
        y: pos.y.rem_euclid(CHUNK_SIZE),
        z: pos.z.rem_euclid(CHUNK_SIZE),
    }
}

fn generate_world(world_size_chunks: i64) -> World {
    let mut world: World = FxHashMap::default();

    let voxel_width = world_size_chunks * CHUNK_SIZE;

    for x in 0..voxel_width {
        for y in 0..60 {
            for z in 0..voxel_width {
                let world_pos = VoxelPos { x, y, z };
                let chunk_pos = voxel_to_chunk_pos(world_pos);
                let local_pos = voxel_to_local_pos(world_pos);

                let chunk = world.entry(chunk_pos).or_insert_with(FxHashMap::default);
                chunk.insert(local_pos, Voxel { kind: NoName });
            }
        }
    }

    world
}

fn is_solid_world(world: &World, pos: VoxelPos) -> bool {
    let chunk_pos = voxel_to_chunk_pos(pos);
    let local_pos = voxel_to_local_pos(pos);

    match world.get(&chunk_pos) {
        Some(chunk) => chunk.contains_key(&local_pos),
        None => false,
    }
}

fn build_chunk_mesh(world: &World, chunk_pos: ChunkPos, thread: &RaylibThread) -> Option<Mesh> {
    let chunk = world.get(&chunk_pos)?;
    if chunk.is_empty() {
        return None;
    }

    let mut verts: Vec<Vector3> = Vec::new();
    let mut uvs: Vec<Vector2> = Vec::new();
    let mut colors: Vec<Color> = Vec::new();

    let mut push_tri = |v0: Vector3, v1: Vector3, v2: Vector3, color: Color| {
        verts.push(v0);
        verts.push(v1);
        verts.push(v2);
        uvs.push(Vector2::new(0.0, 0.0));
        uvs.push(Vector2::new(1.0, 0.0));
        uvs.push(Vector2::new(1.0, 1.0));
        colors.push(color);
        colors.push(color);
        colors.push(color);
    };

    let base_x = chunk_pos.x * CHUNK_SIZE;
    let base_y = chunk_pos.y * CHUNK_SIZE;
    let base_z = chunk_pos.z * CHUNK_SIZE;

    for (&local_pos, _voxel) in chunk.iter() {
        let xf = local_pos.x as f32;
        let yf = local_pos.y as f32;
        let zf = local_pos.z as f32;

        let world_pos = VoxelPos {
            x: base_x + local_pos.x,
            y: base_y + local_pos.y,
            z: base_z + local_pos.z,
        };

        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x,
                y: world_pos.y + 1,
                z: world_pos.z,
            },
        ) {
            let e = Vector3::new(xf, yf + 1.0, zf);
            let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
            let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
            let h = Vector3::new(xf, yf + 1.0, zf + 1.0);
            push_tri(e, g, f, Color::GREEN);
            push_tri(e, h, g, Color::GREEN);
        }
        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x,
                y: world_pos.y - 1,
                z: world_pos.z,
            },
        ) {
            let a = Vector3::new(xf, yf, zf);
            let b = Vector3::new(xf + 1.0, yf, zf);
            let cc = Vector3::new(xf + 1.0, yf, zf + 1.0);
            let d = Vector3::new(xf, yf, zf + 1.0);
            push_tri(a, b, cc, Color::BROWN);
            push_tri(a, cc, d, Color::BROWN);
        }
        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x,
                y: world_pos.y,
                z: world_pos.z - 1,
            },
        ) {
            let a = Vector3::new(xf, yf, zf);
            let b = Vector3::new(xf + 1.0, yf, zf);
            let e = Vector3::new(xf, yf + 1.0, zf);
            let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
            push_tri(a, e, f, Color::BLUE);
            push_tri(a, f, b, Color::BLUE);
        }
        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x,
                y: world_pos.y,
                z: world_pos.z + 1,
            },
        ) {
            let d = Vector3::new(xf, yf, zf + 1.0);
            let cc = Vector3::new(xf + 1.0, yf, zf + 1.0);
            let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
            let h = Vector3::new(xf, yf + 1.0, zf + 1.0);
            push_tri(d, cc, g, Color::ORANGE);
            push_tri(d, g, h, Color::ORANGE);
        }
        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x - 1,
                y: world_pos.y,
                z: world_pos.z,
            },
        ) {
            let a = Vector3::new(xf, yf, zf);
            let d = Vector3::new(xf, yf, zf + 1.0);
            let e = Vector3::new(xf, yf + 1.0, zf);
            let h = Vector3::new(xf, yf + 1.0, zf + 1.0);
            push_tri(a, d, h, Color::PURPLE);
            push_tri(a, h, e, Color::PURPLE);
        }
        if !is_solid_world(
            world,
            VoxelPos {
                x: world_pos.x + 1,
                y: world_pos.y,
                z: world_pos.z,
            },
        ) {
            let b = Vector3::new(xf + 1.0, yf, zf);
            let cc = Vector3::new(xf + 1.0, yf, zf + 1.0);
            let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
            let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
            push_tri(b, f, g, Color::YELLOW);
            push_tri(b, g, cc, Color::YELLOW);
        }
    }

    if verts.is_empty() {
        return None;
    }

    Some(
        Mesh::gen_mesh(&verts, &uvs)
            .colors(&colors)
            .build(thread)
            .unwrap(),
    )
}

fn build_world_meshes(
    world: &World,
    thread: &RaylibThread,
    rl: &mut RaylibHandle,
) -> FxHashMap<ChunkPos, Model> {
    let mut models: FxHashMap<ChunkPos, Model> = FxHashMap::default();

    for &chunk_pos in world.keys() {
        if let Some(mesh) = build_chunk_mesh(world, chunk_pos, thread) {
            let model = unsafe { rl.load_model_from_mesh(thread, mesh.make_weak()).unwrap() };
            models.insert(chunk_pos, model);
        }
    }

    models
}

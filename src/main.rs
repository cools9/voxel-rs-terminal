use raylib::prelude::*;
use raylib::consts::CameraMode;
use std::collections::HashMap;
use rustc_hash::FxHashMap;

use crate::VoxelType::NoName;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]    
struct Voxel{
    kind:VoxelType
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct VoxelPos{
    x:i64,
    y:i64,
    z:i64,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VoxelType{
    NoName
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VoxelFaces{
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right
}

type World = FxHashMap<VoxelPos, Voxel>;

fn main(){
    let (mut rl, thread) = raylib::init()
    .size(1280, 720)
    .title("demo")
    .fullscreen()
    .vsync()
    .build();
    let mut world:World=HashMap::default();
    let world_size:u64=128;
    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    generate_terrain(world_size as i64, &mut world);
    let mesh = build_terrain_mesh(&world, world_size as i64, &thread);
    let model = unsafe{rl.load_model_from_mesh(&thread, mesh.make_weak()).unwrap()};
    

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        camera.update_camera(CameraMode::CAMERA_FREE);
        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);
        d.disable_cursor();
        {
            let mut c = d.begin_mode3D(camera);
            c.draw_model(&model, Vector3::zero(), 1.0, Color::WHITE);
        }
    }
}


fn make_top_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // y = top face (y+1 plane), normal points +y
    let e = Vector3::new(x,       y + 1.0, z);
    let f = Vector3::new(x + 1.0, y + 1.0, z);
    let g = Vector3::new(x + 1.0, y + 1.0, z + 1.0);
    let h = Vector3::new(x,       y + 1.0, z + 1.0);

    c.draw_triangle3D(e, g, f, Color::GREEN);
    c.draw_triangle3D(e, h, g, Color::GREEN);
}

fn make_bottom_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // y = bottom face (y plane), normal points -y
    let a = Vector3::new(x,       y, z);
    let b = Vector3::new(x + 1.0, y, z);
    let cc = Vector3::new(x + 1.0, y, z + 1.0);
    let d = Vector3::new(x,       y, z + 1.0);

    c.draw_triangle3D(a, b, cc, Color::BROWN);
    c.draw_triangle3D(a, cc, d, Color::BROWN);
}

fn make_front_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // z plane (front), normal points -z
    let a = Vector3::new(x,       y,       z);
    let b = Vector3::new(x + 1.0, y,       z);
    let e = Vector3::new(x,       y + 1.0, z);
    let f = Vector3::new(x + 1.0, y + 1.0, z);

    c.draw_triangle3D(a, e, f, Color::BLUE);
    c.draw_triangle3D(a, f, b, Color::BLUE);
}

fn make_back_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // z+1 plane (back), normal points +z
    let d = Vector3::new(x,       y,       z + 1.0);
    let cc = Vector3::new(x + 1.0, y,       z + 1.0);
    let g = Vector3::new(x + 1.0, y + 1.0, z + 1.0);
    let h = Vector3::new(x,       y + 1.0, z + 1.0);

    c.draw_triangle3D(d, cc, g, Color::ORANGE);
    c.draw_triangle3D(d, g, h, Color::ORANGE);
}

fn make_left_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // x plane (left), normal points -x
    let a = Vector3::new(x, y,       z);
    let d = Vector3::new(x, y,       z + 1.0);
    let e = Vector3::new(x, y + 1.0, z);
    let h = Vector3::new(x, y + 1.0, z + 1.0);

    c.draw_triangle3D(a, d, h, Color::PURPLE);
    c.draw_triangle3D(a, h, e, Color::PURPLE);
}

fn make_right_face(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    // x+1 plane (right), normal points +x
    let b = Vector3::new(x + 1.0, y,       z);
    let cc = Vector3::new(x + 1.0, y,       z + 1.0);
    let f = Vector3::new(x + 1.0, y + 1.0, z);
    let g = Vector3::new(x + 1.0, y + 1.0, z + 1.0);

    c.draw_triangle3D(b, f, g, Color::YELLOW);
    c.draw_triangle3D(b, g, cc, Color::YELLOW);
}

fn generate_terrain(world_size:i64,world: &mut World){
    
    for x in 0..world_size{
        for y in 0..60{
            for z in 0..world_size{
                let block_pos= VoxelPos{
                        x:x as i64,
                        y:y as i64,
                        z:z as i64
                };
                world.insert(block_pos, Voxel { kind: NoName });
            }
        }
    }
}

fn make_cube(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32,faces: &[VoxelFaces]) {
    for face in faces{
        match face{
            VoxelFaces::Top => make_top_face(c, x, y, z),
            VoxelFaces::Bottom=>make_bottom_face(c, x, y, z),
            VoxelFaces::Front=>make_front_face(c, x, y, z),
            VoxelFaces::Back=>make_back_face(c, x, y, z),
            VoxelFaces::Left=>make_left_face(c, x, y, z),
            VoxelFaces::Right=>make_right_face(c, x, y, z)
        }
    }
}   

fn is_solid(world: &World, pos: VoxelPos) -> bool {
    world.contains_key(&pos)
}

fn build_terrain_mesh(world: &World, world_size: i64, thread: &RaylibThread) -> Mesh {
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

    for x in 0..world_size {
        for y in 0..60 {
            for z in 0..world_size {
                let pos = VoxelPos { x, y, z };
                if world.get(&pos).is_none() {
                    continue;
                }

                let xf = x as f32;
                let yf = y as f32;
                let zf = z as f32;

                if !is_solid(world, VoxelPos { x: pos.x, y: pos.y + 1, z: pos.z }) {
                    let e = Vector3::new(xf,       yf + 1.0, zf);
                    let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
                    let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
                    let h = Vector3::new(xf,       yf + 1.0, zf + 1.0);
                    push_tri(e, g, f, Color::GREEN);
                    push_tri(e, h, g, Color::GREEN);
                }
                if !is_solid(world, VoxelPos { x: pos.x, y: pos.y - 1, z: pos.z }) {
                    let a = Vector3::new(xf,       yf, zf);
                    let b = Vector3::new(xf + 1.0, yf, zf);
                    let cc = Vector3::new(xf + 1.0, yf, zf + 1.0);
                    let d = Vector3::new(xf,       yf, zf + 1.0);
                    push_tri(a, b, cc, Color::BROWN);
                    push_tri(a, cc, d, Color::BROWN);
                }
                if !is_solid(world, VoxelPos { x: pos.x, y: pos.y, z: pos.z - 1 }) {
                    let a = Vector3::new(xf,       yf,       zf);
                    let b = Vector3::new(xf + 1.0, yf,       zf);
                    let e = Vector3::new(xf,       yf + 1.0, zf);
                    let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
                    push_tri(a, e, f, Color::BLUE);
                    push_tri(a, f, b, Color::BLUE);
                }
                if !is_solid(world, VoxelPos { x: pos.x, y: pos.y, z: pos.z + 1 }) {
                    let d = Vector3::new(xf,       yf,       zf + 1.0);
                    let cc = Vector3::new(xf + 1.0, yf,       zf + 1.0);
                    let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
                    let h = Vector3::new(xf,       yf + 1.0, zf + 1.0);
                    push_tri(d, cc, g, Color::ORANGE);
                    push_tri(d, g, h, Color::ORANGE);
                }
                if !is_solid(world, VoxelPos { x: pos.x - 1, y: pos.y, z: pos.z }) {
                    let a = Vector3::new(xf, yf,       zf);
                    let d = Vector3::new(xf, yf,       zf + 1.0);
                    let e = Vector3::new(xf, yf + 1.0, zf);
                    let h = Vector3::new(xf, yf + 1.0, zf + 1.0);
                    push_tri(a, d, h, Color::PURPLE);
                    push_tri(a, h, e, Color::PURPLE);
                }
                if !is_solid(world, VoxelPos { x: pos.x + 1, y: pos.y, z: pos.z }) {
                    let b = Vector3::new(xf + 1.0, yf,       zf);
                    let cc = Vector3::new(xf + 1.0, yf,       zf + 1.0);
                    let f = Vector3::new(xf + 1.0, yf + 1.0, zf);
                    let g = Vector3::new(xf + 1.0, yf + 1.0, zf + 1.0);
                    push_tri(b, f, g, Color::YELLOW);
                    push_tri(b, g, cc, Color::YELLOW);
                }
            }
        }
    }

    Mesh::gen_mesh(&verts, &uvs)
        .colors(&colors)
        .build(thread)
        .unwrap()
}
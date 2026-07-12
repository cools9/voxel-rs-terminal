use raylib::prelude::*;
//use raylib::consts::CameraMode;

pub mod player;
pub mod world;
use crate::player::*;
use crate::world::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("demo")
        .fullscreen()
        .vsync()
        .build();
    let world_size_chunks: i64 = 8;

    let mut player = player::Player {
        x: 50,
        y: 80,
        z: -100,
        yaw: 0.0,
        pitch: 0.0,
        sensitivity: 0.0003,
    };
    let mut camera = Camera3D::perspective(
        Vector3::new(player.x as f32, player.y as f32, player.z as f32),
        Vector3::new(32.0, 30.0, 32.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let world = world::generate_world(world_size_chunks);
    let models = world::build_world_meshes(&world, &thread, &mut rl);

    let mut direction = movement::NONE;
    while !rl.window_should_close() {
        rl.hide_cursor();
        let mouse_delta = rl.get_mouse_delta();
        //camera.update_camera(CameraMode::CAMERA_FREE);

        direction = if rl.is_key_down(KeyboardKey::KEY_W) {
            movement::FRONT
        } else if rl.is_key_down(KeyboardKey::KEY_S) {
            movement::BACK
        } else if rl.is_key_down(KeyboardKey::KEY_A) {
            movement::LEFT
        } else if rl.is_key_down(KeyboardKey::KEY_D) {
            movement::RIGHT
        } else if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            movement::UP
        } else if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            movement::DOWN
        } else {
            movement::NONE
        };

        match direction {
            movement::FRONT => player.x += 5,
            movement::BACK => player.x -= 5,
            movement::LEFT => player.z -= 5,
            movement::RIGHT => player.z += 5,
            movement::NONE => {}
            movement::UP => player.y += 5,
            movement::DOWN => player.y -= 5,
        }

        player.yaw += mouse_delta.x * player.sensitivity;
        player.pitch = (player.pitch + mouse_delta.y * player.sensitivity).clamp(-2.0, 2.0);
        let forward = Vector3::new(
            player.yaw.cos() * player.pitch.cos(),
            player.pitch.sin(),
            player.yaw.sin() * player.pitch.cos(),
        );
        camera.position = Vector3::new(player.x as f32, player.y as f32, player.z as f32);
        camera.target = camera.position + forward; //Vector3::new(player.x as f32, player.y as f32, player.z as f32 + 1.0);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);

        {
            make_crossair(&mut d);
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

fn make_crossair(d: &mut RaylibDrawHandle) {
    let width: i32 = d.get_screen_width();
    let height: i32 = d.get_screen_height();
    let size_of_crossair: i32 = 10;
    let center_x = width / 2;
    let center_y = height / 2;
    d.draw_line_ex(
        Vector2::new(center_x as f32 - size_of_crossair as f32, center_y as f32),
        Vector2::new(center_x as f32 + size_of_crossair as f32, center_y as f32),
        5.0,
        Color::WHITE,
    );
    d.draw_line_ex(
        Vector2::new(center_x as f32, center_y as f32 - size_of_crossair as f32),
        Vector2::new(center_x as f32, center_y as f32 + size_of_crossair as f32),
        5.0,
        Color::WHITE,
    );
}

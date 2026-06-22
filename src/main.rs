use raylib::ffi::{CSSPalette, RaylibPalette};
use raylib::prelude::*;
use raylib::consts::CameraMode;



fn main(){
    let (mut rl, thread) = raylib::init()
    .size(1280, 720)
    .title("demo")
    .fullscreen()
    .vsync()
    .msaa_4x()
    .build();
    let world_size:u64=128;
    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    while !rl.window_should_close() {
        
        let mut d = rl.begin_drawing(&thread);
        d.rl_disable_backface_culling();
        camera.update_camera(CameraMode::CAMERA_FREE);
        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);
        d.disable_cursor();
        {
            let mut c = d.begin_mode3D(camera);
            //for x in 0..world_size{
                //for z in 0..world_size
                c.draw_grid(20, 1.0);
                make_cube(&mut c, 0.0, 0.0, 0.0);
                    //c.draw_cube(Vector3::new(x as f32,0.0,z as f32), 1.0, 1.0, 1.0, Color::RED);
                //}
            //}
            //c.draw_grid(10, 1.0);
            // Drop ends 3D mode automatically.
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

fn make_cube(c: &mut RaylibMode3D<RaylibDrawHandle>, x: f32, y: f32, z: f32) {
    make_top_face(c, x, y, z);
    make_bottom_face(c, x, y, z);
    make_front_face(c, x, y, z);
    make_back_face(c, x, y, z);
    make_left_face(c, x, y, z);
    make_right_face(c, x, y, z);
}
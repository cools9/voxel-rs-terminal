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
        camera.update_camera(CameraMode::CAMERA_FIRST_PERSON);
        d.clear_background(Color::RAYWHITE);
        d.draw_fps(0, 0);
        d.hide_cursor();
        {
            let mut c = d.begin_mode3D(camera);
            for x in 0..world_size{
                for z in 0..world_size{
                    c.draw_cube(Vector3::new(x as f32,0.0,z as f32), 1.0, 1.0, 1.0, Color::RED);
                }
            }
            //c.draw_grid(10, 1.0);
            // Drop ends 3D mode automatically.
        }
    }
}
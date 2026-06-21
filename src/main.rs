use raylib::prelude::*;

fn main(){
    let (mut rl, thread) = raylib::init().size(800, 600).title("demo").build();
    let camera = Camera3D::perspective(
        Vector3::new(4.0, 4.0, 4.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        {
            let mut c = d.begin_mode3D(camera);
            c.draw_cube(Vector3::zero(), 2.0, 2.0, 2.0, Color::RED);
            c.draw_grid(10, 1.0);
            // Drop ends 3D mode automatically.
        }
        d.draw_text("hello 3D", 10, 10, 20, Color::BLACK);
    }
}
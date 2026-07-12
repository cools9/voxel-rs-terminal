pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}
pub enum movement {
    FRONT,
    BACK,
    LEFT,
    RIGHT,
    NONE,
    UP,
    DOWN,
}

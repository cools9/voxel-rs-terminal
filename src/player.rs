pub struct Player {
    pub x: i64,
    pub y: i64,
    pub z: i64,
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

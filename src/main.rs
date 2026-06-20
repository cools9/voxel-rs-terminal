/*
const SIZE: usize = 16;

struct World {
    voxels: [[[bool; SIZE]; SIZE]; SIZE],
}
*/

struct Player {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
}

struct Block{
    x:f32,
    y:f32,
    z:f32
}

fn main(){
  let mut player= Player{
    x:0.0,
    y:0.0,
    z:0.0,
    yaw:0.0,
    pitch:0.0
  };
  let mut block=Block { x:0.0,
     y:0.0,
      z:5.0 };

    let dx=block.x-player.x;
    let dy= block.y-player.y;
    let dz= block.z-player.z;
    let distance=(dx * dx + dy * dy + dz * dz).sqrt();
    let size = (20.0 / distance) as u16;
    println!("{}",size);
}
use crate::vector::Point;
use minifb::Key;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,

    Forward,
    Backward,

    PitchUp,
    PitchDown,
    YawLeft,
    YawRight,
}

pub struct Camera {
    pub pos: Point<f32>,
    pub dir: Point<f32>,
    pub fov: f32,
    pub fow_tan: f32,
}

impl Camera {
    pub fn new(pos: Point<f32>, dir: Point<f32>, fov: f32) -> Self {
        Self {
            pos,
            dir: dir.to_normalized(),
            fov,
            fow_tan: (fov * 0.5).tan(),
        }
    }

    pub fn move_to(&mut self, dir: Direction, amount: f32) {
        match dir {
            Direction::Up => self.pos.y += amount,
            Direction::Down => self.pos.y -= amount,
            Direction::Left => self.pos.x -= amount,
            Direction::Right => self.pos.x += amount,
            Direction::Forward => self.pos += self.dir * amount,
            Direction::Backward => self.pos -= self.dir * amount,
            Direction::PitchUp => self.dir.y += amount,
            Direction::PitchDown => self.dir.y -= amount,
            Direction::YawLeft => self.dir.x -= amount,
            Direction::YawRight => self.dir.x += amount,
        }
    }
    pub fn keyboard(&mut self, key: Key) {
        let move_speed = 10.0;
        let yaw_pitch_speed = 0.1;

        match key {
            Key::W => self.move_to(Direction::Up, move_speed),
            Key::S => self.move_to(Direction::Down, move_speed),
            Key::A => self.move_to(Direction::Left, move_speed),
            Key::D => self.move_to(Direction::Right, move_speed),
            Key::F => self.move_to(Direction::Forward, move_speed),
            Key::B => self.move_to(Direction::Backward, move_speed),
            Key::Up => self.move_to(Direction::PitchUp, yaw_pitch_speed),
            Key::Down => self.move_to(Direction::PitchDown, yaw_pitch_speed),
            Key::Left => self.move_to(Direction::YawLeft, yaw_pitch_speed),
            Key::Right => self.move_to(Direction::YawRight, yaw_pitch_speed),

            Key::P => println!("pos: {:?}", self.pos),
            _ => {}
        }
    }
}

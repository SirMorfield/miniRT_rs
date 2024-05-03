use crate::vector::Point;
use minifb::Key;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Camera {
    pub pos: Point<f32>,
    pub dir: Point<f32>,
    pub fov: f32,
    pub fow_tan: f32,
    move_speed: f32,
    yaw_pitch_speed: f32,
}

impl Camera {
    pub fn new(pos: Point<f32>, dir: Point<f32>, fov: f32, move_speed: f32, yaw_pitch_speed: f32) -> Self {
        // let pos = Point {
        Self {
            pos,
            dir: dir.to_normalized(),
            fov,
            fow_tan: (fov * 0.5).tan(),
            move_speed,
            yaw_pitch_speed,
        }
    }

    pub fn update_pitch_yaw(&mut self, pitch_delta: f32, yaw_delta: f32) {
        let pitch = f32::asin(self.dir.y) + pitch_delta;
        let yaw = f32::atan2(self.dir.x, self.dir.z) + yaw_delta;

        self.dir = Point::new(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos()).to_normalized();
    }

    pub fn update_right_left(&mut self, delta: f32) {
        let left = self.dir.cross(&Point::new(0.0, 1.0, 0.0)).to_normalized();
        self.pos += left * delta;
    }

    pub fn move_to(&mut self, dir: Direction, amount: f32) {
        match dir {
            Direction::Up => self.pos.y += amount,
            Direction::Down => self.pos.y -= amount,
            Direction::Left => self.update_right_left(amount),
            Direction::Right => self.update_right_left(-amount),
            Direction::Forward => self.pos += self.dir * amount,
            Direction::Backward => self.pos -= self.dir * amount,
            Direction::PitchUp => self.update_pitch_yaw(amount, 0.0),
            Direction::PitchDown => self.update_pitch_yaw(-amount, 0.0),
            Direction::YawLeft => self.update_pitch_yaw(0.0, amount),
            Direction::YawRight => self.update_pitch_yaw(0.0, -amount),
        }
    }

    // returns true if the camera was moved
    pub fn keyboard(&mut self, key: &Key) -> bool {
        let mut moved = true;
        match key {
            Key::W => self.move_to(Direction::Up, self.move_speed),
            Key::S => self.move_to(Direction::Down, self.move_speed),
            Key::A => self.move_to(Direction::Left, self.move_speed),
            Key::D => self.move_to(Direction::Right, self.move_speed),
            Key::F => self.move_to(Direction::Forward, self.move_speed),
            Key::B => self.move_to(Direction::Backward, self.move_speed),
            Key::Up => self.move_to(Direction::PitchUp, self.yaw_pitch_speed),
            Key::Down => self.move_to(Direction::PitchDown, self.yaw_pitch_speed),
            Key::Left => self.move_to(Direction::YawLeft, self.yaw_pitch_speed),
            Key::Right => self.move_to(Direction::YawRight, self.yaw_pitch_speed),

            Key::P => println!("pos: {:?}\ndir: {:?}", self.pos, self.dir),
            _ => moved = false,
        }
        return moved;
    }
}

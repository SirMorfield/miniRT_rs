use crate::vector::Vec3;
use minifb::Key;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}

pub struct Camera {
    pub pos: Vec3<f32>,
    pub dir: Vec3<f32>,
    pub fov: f32,
    pub fow_tan: f32,
}

impl Camera {
    pub fn new(pos: Vec3<f32>, dir: Vec3<f32>, fov: f32) -> Self {
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
        }
    }
    pub fn move_from_keyboard(&mut self, key: Key, speed: f32) {
        match key {
            Key::W => self.move_to(Direction::Forward, speed),
            Key::S => self.move_to(Direction::Backward, speed),
            Key::A => self.move_to(Direction::Left, speed),
            Key::D => self.move_to(Direction::Right, speed),
            Key::Up => self.move_to(Direction::Up, speed),
            Key::Down => self.move_to(Direction::Down, speed),
            _ => {}
        }
    }
}

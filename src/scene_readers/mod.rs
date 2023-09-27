use crate::{camera::Camera, light::Light, vector::Vec3};

pub mod rt;

pub trait SceneTrait {
    fn camera(&self) -> &Camera;
    fn lights(&self) -> &Vec<Light>;
    fn ambient(&self) -> &Light;
    fn void(&self) -> Vec3<u8>;
}

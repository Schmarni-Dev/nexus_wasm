#[repr(C)]
#[derive(Debug)]
pub struct Transform {
    pub pos: Vec3,
    pub rotation: Quat,
}

#[repr(C)]
#[derive(Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

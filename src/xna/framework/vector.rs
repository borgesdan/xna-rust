use crate::xna::framework::{Vector2, Vector3, Vector4};

impl Vector2 {

    pub fn from_xy(x: f32, y: f32) -> Vector2 { Vector2 { x, y }}

    pub fn zero() -> Vector2 { Vector2::default() }

    pub fn one() -> Vector2 { Vector2 { x: 1.0, y: 1.0 }}

    pub fn unit_x() -> Vector2 { Vector2 { x: 1.0, y: 0.0 }}

    pub fn unit_y() -> Vector2 { Vector2 { x: 0.0, y: 1.0 }}

}

impl Vector3 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Vector3 { Vector3 { x, y, z }}

    pub fn zero() -> Vector3 { Vector3::default() }

    pub fn one() -> Vector3 { Vector3 { x: 1.0, y: 1.0, z: 1.0 }}

    pub fn unit_x() -> Vector3 { Vector3 { x: 1.0, y: 0.0, z: 0.0 }}

    pub fn unit_y() -> Vector3 { Vector3 { x: 0.0, y: 1.0, z: 0.0 }}

    pub fn unit_z() -> Vector3 { Vector3 { x: 0.0, y: 0.0, z: 1.0 }}

    pub fn up() -> Vector3 { Self::unit_y() }

    pub fn down() -> Vector3 {Vector3 { x: 0.0, y: -1.0, z: 0.0 } }

    pub fn right() -> Vector3 { Self::unit_x() }

    pub fn left() -> Vector3 {Vector3 { x: -1.0, y: 0.0, z: 0.0 } }

    pub fn forward() -> Vector3 {Vector3 { x: 0.0, y: 0.0, z: -1.0 } }

    pub fn backward() -> Vector3 { Self::unit_z() }
}

impl Vector4 {
    pub fn from_xyzw(x: f32, y: f32, z: f32, w:f32) -> Vector4 { Vector4 { x, y, z, w }}

    pub fn zero() -> Vector4 { Vector4::default() }

    pub fn one() -> Vector4 { Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }}

    pub fn unit_x() -> Vector4 { Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0 }}

    pub fn unit_y() -> Vector4 { Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 1.0 }}

    pub fn unit_z() -> Vector4 { Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0 }}

    pub fn unit_w() -> Vector4 { Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0 }}
}
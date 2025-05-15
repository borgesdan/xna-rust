pub mod game;
pub mod graphics;
pub mod color;
pub mod vector;
pub mod point;
mod rectangle;
pub mod content;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Color {
    pub packed_value: u32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct Plane {
    pub normal: Vector3,
    pub d: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct BoundingSphere {
    pub center: Vector3,
    pub radius: f32,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub struct BoundingFrustum {
}
use crate::xna::framework::{Vector3, Vector4};
use crate::xna::framework::graphics::Alpha8;
use crate::xna::framework::graphics::PackUtils;
use crate::xna::framework::graphics::IPackedVector;
use crate::xna::framework::graphics::Bgr565;

impl PackUtils {
    pub fn clamp_and_round(value: f32, min: f32, max: f32) -> f64 {
        if value.is_nan() {
            return 0.0
        }

        if value.is_infinite() {
            if value.is_sign_negative() {
                return min as f64
            }

            return max as f64
        }

        if value < min{
            return min as f64
        }

        if value > max{
            return max as f64
        }

        value.round() as f64
    }

    pub fn unpack_snorm(bitmask: u32, value: u32) -> f32 {
        let num1 = bitmask + 1u32 >> 1;
        let value2: u32;

        if((value as i32) & (num1 as i32)) != 0 {
            if (value as i32) & (bitmask as i32) == num1 as i32 {
                return -1.0;
            }

            value2 = value | !bitmask;
        } else {
            value2 = value & bitmask;
        }

        let num2 = (bitmask >> 1) as f32;

        (value2 as f32) / num2
    }

    pub fn pack_snorm(bitmask: u32, value: f32) -> u32 {
        let max = (bitmask >> 1) as f32;
        let value2 = value * max;

        (Self::clamp_and_round(value2, -max, max) as u32) & bitmask
    }

    pub fn unpack_unorm(bitmask: u32, value: u32) -> f32 {
        let result = value & bitmask;
        (result as f32) / (bitmask as f32)
    }

    pub fn pack_unorm(bitmask: f32, value: f32) -> u32 {
        let result = value * bitmask;
        Self::clamp_and_round(result, 0f32, bitmask) as u32
    }

    pub fn pack_signed(bitmask: u32, value: f32) -> u32 {
        let max = (bitmask >> 1) as f32;
        let min = -max - 1.0;

        (Self::clamp_and_round(value, min, max) as u32) & bitmask
    }

    pub fn pack_unsigned(bitmask: f32, value: f32) -> u32 {
        Self::clamp_and_round(value, 0f32, bitmask) as u32
    }
}

impl Alpha8 {
    pub fn from_alpha(alpha: f32) -> Alpha8 {
        let packed_value = PackUtils::pack_unorm(u8::MAX as f32, alpha) as u8;
        Alpha8 { packed_value }
    }
}

impl IPackedVector for Alpha8 {
    fn to_vector4(&self) -> Vector4 {
        let alpha = PackUtils::unpack_unorm(u8::MAX as u32, self.packed_value as u32);

        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: alpha,
        }
    }
}

impl Bgr565 {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Bgr565 {
        let packed_value = (PackUtils::pack_unorm(31.0, x) << 11
        | PackUtils::pack_unorm(63.0, y) << 5
        | PackUtils::pack_unorm(31.0, z)) as u16;

        Bgr565{ packed_value }
    }

    pub fn from_vector3(vector: Vector3) -> Bgr565 {
        let packed_value = (PackUtils::pack_unorm(31.0, vector.x) << 11
            | PackUtils::pack_unorm(63.0, vector.y) << 5
            | PackUtils::pack_unorm(31.0, vector.z)) as u16;

        Bgr565{ packed_value }
    }

    pub fn to_vector3(&self) -> Vector3 {
        let x = PackUtils::unpack_unorm(31, self.packed_value as u32 >> 11);
        let y= PackUtils::unpack_unorm(63, self.packed_value as u32 >> 5);
        let z= PackUtils::unpack_unorm(31, self.packed_value as u32);

        Vector3{ x, y, z}
    }
}

impl IPackedVector for Bgr565 {
    fn to_vector4(&self) -> Vector4 {
        let vector = self.to_vector3();

        Vector4 {
            x: vector.x,
            y: vector.y,
            z: vector.z,
            w: 1.0,
        }
    }
}

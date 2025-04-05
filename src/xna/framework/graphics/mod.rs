use crate::xna::framework::Vector4;

pub trait IPackedVector {
    fn to_vector4(&self) -> Vector4;
    fn from_vector4(vector4: &Vector4);
}

pub trait ITPackedVector<TPacked>{
    fn packed_value(&self) -> TPacked;
    fn set_packed_value(value: &TPacked);
}

pub struct PackUtils {}

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
        let mut value2: u32;

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
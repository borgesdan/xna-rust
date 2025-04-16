use std::iter::FilterMap;
use crate::xna::framework::graphics::{SamplerState, TextureAddressMode, TextureFilter};

impl SamplerState {
    pub fn point_wrap() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Point,
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            ..Default::default()
        }
    }

    pub fn point_clamp() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Point,
            address_u: TextureAddressMode::Clamp,
            address_v: TextureAddressMode::Clamp,
            address_w: TextureAddressMode::Clamp,
            ..Default::default()
        }
    }

    pub fn linear_wrap() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Linear,
            address_u: TextureAddressMode::Clamp,
            address_v: TextureAddressMode::Clamp,
            address_w: TextureAddressMode::Clamp,
            ..Default::default()
        }
    }

    pub fn linear_clamp() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Linear,
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            ..Default::default()
        }
    }

    pub fn anisotropic_wrap() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Anisotropic,
            address_u: TextureAddressMode::Clamp,
            address_v: TextureAddressMode::Clamp,
            address_w: TextureAddressMode::Clamp,
            ..Default::default()
        }
    }

    pub fn anisotropic_clamp() -> SamplerState {
        SamplerState {
            filter: TextureFilter::Anisotropic,
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            ..Default::default()
        }
    }
}
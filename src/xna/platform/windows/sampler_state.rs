use windows::Win32::Graphics::Direct3D11::D3D11_SAMPLER_DESC;
use crate::xna::framework::graphics::{IPackedVector, SamplerState};

impl SamplerState {
    pub fn to_dx(&self) -> D3D11_SAMPLER_DESC {
        let border_color = self.border_color.to_vector4();

        D3D11_SAMPLER_DESC {
            AddressU: self.address_u.to_dx(),
            AddressV: self.address_v.to_dx(),
            AddressW: self.address_w.to_dx(),
            BorderColor: [border_color.x, border_color.y, border_color.z, border_color.w],
            ComparisonFunc: self.comparison_function.to_dx(),
            Filter: self.filter.to_dx(),
            MaxAnisotropy: self.max_anisotropy,
            MaxLOD: self.max_mip_level,
            MinLOD: self.min_mip_level,
            MipLODBias: self.mip_map_level_of_detail_bias,
        }
    }
}
use windows::Win32::Graphics::Direct3D11::{ID3D11ShaderResourceView, ID3D11Texture2D, D3D11_BIND_SHADER_RESOURCE, D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT};
use windows::Win32::Graphics::Direct3D::D3D10_1_SRV_DIMENSION_TEXTURE2D;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC};
use crate::xna::framework::graphics::Texture2D;
use crate::xna::platform::windows::graphics_device::WindowsGraphicsDevice;

#[derive(Default, Clone)]
pub struct WindowsTexture2D {
    pub texture: Option<ID3D11Texture2D>,
    pub shader: Option<ID3D11ShaderResourceView>,
    pub sub_resource: D3D11_SUBRESOURCE_DATA,
    pub description: D3D11_TEXTURE2D_DESC,
    pub shared_description: D3D11_SHADER_RESOURCE_VIEW_DESC,
    pub base: Texture2D
}

impl Texture2D {
    pub fn create(&self) -> WindowsTexture2D {
        WindowsTexture2D {
            description: D3D11_TEXTURE2D_DESC {
                MipLevels: 1,
                ArraySize: 1,
                Format: self.format.to_dx(),
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    ..Default::default()
                },
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                Width: self.width,
                Height: self.height,
                ..Default::default()
            },
            shared_description: D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ViewDimension: D3D10_1_SRV_DIMENSION_TEXTURE2D,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
use crate::xna::framework::graphics::{GraphicsDevice, RenderTarget2D};
use windows::Win32::Graphics::Direct3D11::{ID3D11Texture2D, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION, D3D11_RTV_DIMENSION_TEXTURE2D, D3D11_TEXTURE2D_DESC};

impl RenderTarget2D {
    pub fn from_back_buffer(&self, g_device: &GraphicsDevice) -> RenderTarget2D {
        let swap_chain = g_device.platform.swap_chain.as_ref().unwrap();
        unsafe {
            let result = swap_chain.GetBuffer::<ID3D11Texture2D>(0).unwrap();

            let mut target = RenderTarget2D::default();
            target.platform.texture = Some(result);

            target
        }
    }

    pub fn initialize(&mut self, device: &GraphicsDevice) {
        unsafe {
            let w_device = device.platform.device.as_ref().unwrap();
            let resource = self.platform.texture.clone();

            let mut texture_desc =D3D11_TEXTURE2D_DESC::default();
            resource.as_ref().unwrap().GetDesc(&mut texture_desc);

            let description = D3D11_RENDER_TARGET_VIEW_DESC {
                ViewDimension: Self::VIEW_DIMENSION,
                Format: texture_desc.Format,
                ..Default::default()
            };

            w_device.CreateRenderTargetView(resource.as_ref().unwrap(), Some(&description), Some(&mut self.platform.view))
                .unwrap();

            let mut description = D3D11_RENDER_TARGET_VIEW_DESC::default();
            self.platform.view.as_ref().unwrap().GetDesc(&mut description);
        }
    }

    pub const VIEW_DIMENSION: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION_TEXTURE2D;
}
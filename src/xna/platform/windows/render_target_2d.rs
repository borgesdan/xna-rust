use crate::xna::framework::graphics::{GraphicsDevice, RenderTarget2D};
use windows::Win32::Graphics::Direct3D11::{ID3D11Texture2D, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION, D3D11_RTV_DIMENSION_TEXTURE2D, D3D11_TEXTURE2D_DESC};
use crate::xna::csharp::{Exception, ExceptionConverter};
use crate::xna::Unbox;

impl RenderTarget2D {
    pub fn from_back_buffer(g_device: &GraphicsDevice) -> Result<RenderTarget2D, Exception> {
        let swap_chain = g_device.platform.swap_chain.unbox_ref()?;

        unsafe {
            let result = swap_chain.GetBuffer::<ID3D11Texture2D>(0)
                .unwrap_or_exception("GetBuffer failed")?;

            let mut target = RenderTarget2D::default();
            target.platform.texture = Some(result);

            Ok(target)
        }
    }

    pub fn initialize(&mut self, device: &GraphicsDevice) -> Result<(), Exception> {
        unsafe {
            let w_device = device.platform.device.unbox_ref()?;
            let resource = self.platform.texture.clone();

            let mut texture_desc = D3D11_TEXTURE2D_DESC::default();
            resource.unbox_ref()?.GetDesc(&mut texture_desc);

            let description = D3D11_RENDER_TARGET_VIEW_DESC {
                ViewDimension: Self::VIEW_DIMENSION,
                Format: texture_desc.Format,
                ..Default::default()
            };

            w_device.CreateRenderTargetView(resource.as_ref().unwrap(), Some(&description), Some(&mut self.platform.view))
                .unwrap_or_exception("CreateRenderTargetView failed")?;

            let mut description = D3D11_RENDER_TARGET_VIEW_DESC::default();
            self.platform.view.as_ref().unwrap().GetDesc(&mut description);

            Ok(())
        }
    }

    pub const VIEW_DIMENSION: D3D11_RTV_DIMENSION = D3D11_RTV_DIMENSION_TEXTURE2D;
}
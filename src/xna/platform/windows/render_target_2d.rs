use windows::Win32::Graphics::Direct3D11::{ID3D11RenderTargetView, ID3D11Texture2D, D3D11_RENDER_TARGET_VIEW_DESC};
use windows::Win32::Graphics::Dxgi::IDXGISwapChain1;
use crate::xna::framework::graphics::RenderTarget2D;
use crate::xna::platform::windows::graphics_device::WindowsGraphicsDevice;
use crate::xna::platform::windows::texture_2d::WindowsTexture2D;

#[derive(Default, Clone)]
pub struct WindowsRenderTarget2D {
    pub view: Option<ID3D11RenderTargetView>,
    pub description: D3D11_RENDER_TARGET_VIEW_DESC,
    pub base: WindowsTexture2D,
}

impl RenderTarget2D {
    pub fn from_back_buffer(&self, g_device: &WindowsGraphicsDevice) -> WindowsRenderTarget2D {
        let swap_chain = g_device.swap_chain.as_ref().unwrap();
        unsafe {
            let result = swap_chain.GetBuffer::<ID3D11Texture2D>(0).unwrap();

            WindowsRenderTarget2D {
                base: WindowsTexture2D {
                    texture: Some(result),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

impl WindowsRenderTarget2D {
    pub fn initialize(&mut self, device: &WindowsGraphicsDevice) {
        let w_device = device.device.as_ref().unwrap();
        let resource = self.base.texture.as_ref().unwrap();

        unsafe {
            w_device.CreateRenderTargetView(resource, Some(&self.description), Some(&mut self.view))
                .unwrap();

            let mut description = D3D11_RENDER_TARGET_VIEW_DESC::default();
            self.view.as_ref().unwrap().GetDesc(&mut description);

            self.description = description;
        }
    }
}
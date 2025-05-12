use crate::xna::framework::graphics::{GraphicsDevice, SwapChain};
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};

impl SwapChain {
    pub fn initialize(&self, device: &GraphicsDevice) -> Option<IDXGISwapChain> {
        let mut desc = DXGI_SWAP_CHAIN_DESC::from(self.clone());
        desc.OutputWindow = device.presentation_parameters.platform.hwnd.clone();

        let factory = device.platform.factory.as_ref().unwrap();
        let i_device = device.platform.device.as_ref().unwrap();
        let mut swap_chain: Option<IDXGISwapChain> = None;
        unsafe{
            factory.CreateSwapChain(i_device, &desc, &mut swap_chain).unwrap();
        }

        return swap_chain;
    }
}
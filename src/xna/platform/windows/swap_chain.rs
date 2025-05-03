use crate::xna::framework::graphics::{GraphicsDevice, SwapChain};
use crate::xna::platform::windows::bool_to_win_bool;
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};

impl SwapChain {
    pub fn to_dx(&self) -> DXGI_SWAP_CHAIN_DESC {
        DXGI_SWAP_CHAIN_DESC {
            Windowed: bool_to_win_bool(self.windowed),
            BufferCount: self.buffer_count,
            Flags: self.flags.to_dx().0 as u32,
            BufferDesc: self.display.to_dx(),
            SwapEffect: self.swap_effect.to_dx(),
            BufferUsage: self.usage.to_dx(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: self.sample_count,
                Quality: self.sample_quality,
            },
            ..Default::default()
        }
    }

    pub fn initialize(&self, device: &GraphicsDevice) -> Option<IDXGISwapChain> {
        let mut desc = self.to_dx();

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
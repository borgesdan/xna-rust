use windows::core::{IUnknown, Interface};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::Win32::Graphics::Dxgi::{IDXGIDevice, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use crate::xna::framework::graphics::SwapChain;
use crate::xna::platform::windows::bool_to_win_bool;
use crate::xna::platform::windows::graphics_device::WindowsGraphicsDevice;

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

    pub fn initialize(&self, w_device: &WindowsGraphicsDevice) -> Option<IDXGISwapChain> {
        let mut desc = self.to_dx();

        desc.OutputWindow = w_device.parameters.hwnd.clone();
        let factory = w_device.factory.as_ref().unwrap();
        let device= w_device.device.as_ref().unwrap();
        let mut swap_chain: Option<IDXGISwapChain> = None;
        unsafe{
            factory.CreateSwapChain(device, &desc, &mut swap_chain).unwrap();
        }

        return swap_chain;
    }
}
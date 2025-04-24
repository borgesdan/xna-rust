use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::Win32::Graphics::Dxgi::DXGI_SWAP_CHAIN_DESC;
use crate::xna::framework::graphics::SwapChain;
use crate::xna::platform::windows::bool_to_win_bool;

impl SwapChain {
    pub fn to_dx(&self) -> DXGI_SWAP_CHAIN_DESC {
        DXGI_SWAP_CHAIN_DESC {
            Windowed: bool_to_win_bool(self.windowed),
            BufferCount: self.buffer_count,
            Flags: self.flags as u32,
            BufferDesc: self.display.to_dx(),
            SwapEffect: self.swap_effect.to_dx(),
            BufferUsage: self.usage.to_dx(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: self.sample_count,
                Quality: self.sample_count,
            },
            ..Default::default()
        }
    }
}
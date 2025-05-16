use crate::xna::framework::graphics::{GraphicsDevice, SwapChain};
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use crate::xna::csharp::Exception;
use crate::xna::{SilentExceptionConverter};

impl SwapChain {
    pub fn initialize(&self, device: &GraphicsDevice) -> Result<Option<IDXGISwapChain>, Exception> {
        let mut desc = DXGI_SWAP_CHAIN_DESC::from(self.clone());
        desc.OutputWindow = device.presentation_parameters.platform.hwnd.clone();

        if desc.OutputWindow.is_invalid() {
            return Err(Exception::new("The device.presentation_parameters.platform.hwnd is invalid.", None));
        }

        let factory = device.platform.factory.unwrap_ref_or_default_exception()?;
        let i_device = device.platform.device.unwrap_ref_or_default_exception()?;
        let mut swap_chain: Option<IDXGISwapChain> = None;

        unsafe{
            let result = factory.CreateSwapChain(i_device, &desc, &mut swap_chain);

            if result.is_err() {
                let code = result.0.to_string();
                let message = result.message();

                let final_message =  "CreateSwapChain failed. - ".to_string() + code.as_str() + " - " + message.as_str();

                return Err(Exception::new(final_message.as_str(), None));
            }
        }

        Ok(swap_chain)
    }
}
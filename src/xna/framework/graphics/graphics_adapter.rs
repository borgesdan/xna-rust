use crate::xna::framework::graphics::GraphicsAdapter;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Com::*;

impl GraphicsAdapter {
    pub fn adapters(adapters: &mut Vec<GraphicsAdapter>){
        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap();

            let mut count: u32 = 0;

            loop {
                let adapter = factory.EnumAdapters(count);

                if adapter.is_err()
                {
                    break;
                }

                let adapter = adapter.unwrap();

                let description = adapter.GetDesc().unwrap();

                let mut adp = GraphicsAdapter::default();
                adp.index = count;
                adp.device_id = description.DeviceId;
                adp.is_default = count == 0;
                adp.revision = description.Revision;
                adp.sub_system_id = description.SubSysId;
                adp.vendor_id = description.VendorId;
                adp.description = String::from_utf16(&description.Description).unwrap();

                Self::set_output_vars(&adapter, &mut adp);

                adapters.push(adp);

                count += 1;
            }
        }
    }

    fn set_output_vars(dx_adapter: &IDXGIAdapter, adapter: &mut GraphicsAdapter){
         unsafe {
             let output = dx_adapter.EnumOutputs(0);

             if output.is_err(){
                 return;
             }

             let output = output.unwrap();
             let mut description = output.GetDesc().unwrap();

             adapter.device_name = String::from_utf16(&description.DeviceName).unwrap();
             adapter.monitor_handle = description.Monitor.0 as isize;
         }
    }
}
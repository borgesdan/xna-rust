use std::ptr;
use std::boxed;
use windows::core::Param;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D;
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_DEBUG, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIFactory1};
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice};

pub struct WindowsGraphicsDevice {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext,
    pub factory: IDXGIFactory,
    pub feature_level: D3D_FEATURE_LEVEL,
    pub base: GraphicsDevice
}

impl GraphicsDevice {
    pub fn create() -> WindowsGraphicsDevice{
        unsafe {
            let flags = D3D11_CREATE_DEVICE_DEBUG;
            let hmodule = HMODULE::default();
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap();
            //let adapter = factory.EnumAdapters(0).unwrap();
            //let adp: Option<IDXGIAdapter> = None;

            let mut device: Option<ID3D11Device> = None;
            let mut context: Option<ID3D11DeviceContext> = None;

            let feature_levels = [
                D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_10_1,
                D3D_FEATURE_LEVEL_10_0,
                D3D_FEATURE_LEVEL_9_3,
                D3D_FEATURE_LEVEL_9_2,
                D3D_FEATURE_LEVEL_9_1,
            ];

            let mut feature_level = D3D_FEATURE_LEVEL_11_0;

            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                hmodule,
                flags,
                Some(&feature_levels),
                D3D11_SDK_VERSION,
                Some(&mut device),
                Some(&mut feature_level),
                Some(&mut context)
            ).unwrap();

            WindowsGraphicsDevice {
                context: context.unwrap(),
                device: device.unwrap(),
                factory: factory,
                feature_level: feature_level,
                base: GraphicsDevice::default(),
            }
        }
    }

    pub fn initialize() {
    }
}

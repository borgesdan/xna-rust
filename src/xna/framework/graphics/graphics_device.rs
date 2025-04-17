use std::ptr;
use std::boxed;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D;
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_DEBUG, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIFactory1};
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice};

pub struct WindowsGraphicsDevice {
    pub device: Box<ID3D11Device>,
    pub context: Box<ID3D11DeviceContext>,
    pub factory: Box<IDXGIFactory>,
    pub graphics_device: GraphicsDevice
}

impl GraphicsDevice {
    fn create() {
        unsafe {
            let flags = D3D11_CREATE_DEVICE_DEBUG;
            let hmodule = HMODULE::default();
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap();
            let adapter = factory.EnumAdapters(0).unwrap();

            let mut other: Box<Option<ID3D11Device>> = Box::default();
            //let adapter: *mut Option<IDXGIAdapter> = ptr::null_mut();
            let device: *mut Option<ID3D11Device> = ptr::null_mut();
            let context: *mut Option<ID3D11DeviceContext> = ptr::null_mut();

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
                Some(device),
                Some(&mut feature_level),
                Some(context)
            ).unwrap();
        }
    }

    pub fn initialize() {
    }
}

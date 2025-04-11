use std::ffi::c_void;
use std::mem;
use windows::core::PCSTR;
use std::ptr;
use windows::Win32::Graphics::Gdi::{CreateDCA, HDC, HMONITOR };
use windows::Win32::Graphics::Gdi::MONITORINFOEXA;
use windows::Win32::Graphics::Gdi::GetMonitorInfoA;
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;
use crate::xna::csharp::{Rectangle };
use crate::xna::csharp::forms::Screen;

impl Screen {
    pub fn from_monitor(monitor: isize, hdc: isize) -> Self {
        //TODO: multi monitor support

        let h_monitor = HMONITOR(monitor as *mut c_void);
        let info = Self::get_monitor_info(&h_monitor);

        let bounds = Rectangle::from_ltrb(
            info.monitorInfo.rcMonitor.left,
            info.monitorInfo.rcMonitor.top,
            info.monitorInfo.rcMonitor.right,
            info.monitorInfo.rcMonitor.bottom
        );

        let screen_dc: HDC;
        let device_name = info.szDevice;

        if(hdc == 0) {
            unsafe {
                let driver = PCSTR(ptr::null());
                let device = PCSTR(device_name.as_ptr() as *const u8);
                let output = PCSTR(ptr::null());
                screen_dc = CreateDCA(driver, device, output, None);
            }
        }

        let working_area = Rectangle::from_ltrb(
            info.monitorInfo.rcWork.left,
            info.monitorInfo.rcWork.top,
            info.monitorInfo.rcWork.right,
            info.monitorInfo.rcWork.bottom,
        );

        let primary = (info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

        let mut name: [u8; 32] = [0; 32];

        let mut index = 0;
        for i in device_name {
            name[index] = i as u8;
            index += 1;
        }

        Screen {
            h_monitor: monitor,
            device_name: String::from_utf8(name.to_vec()).unwrap(),
            bounds,
            primary,
            working_area
        }
    }

    pub fn all_screens() -> Vec<Self> {
        //TODO: get all screens
        let primary_monitor = 65537;
        let primary_screen = Screen::from_monitor(primary_monitor, 0);
        let mut screens = Vec::<Screen>::new();
        screens.push(primary_screen);

        screens
    }

    fn get_monitor_info(monitor: &HMONITOR) -> MONITORINFOEXA {
        let mut monitor_info: MONITORINFOEXA = unsafe { mem::zeroed() };
        monitor_info.monitorInfo.cbSize = size_of::<MONITORINFOEXA>() as u32;
        
        unsafe {
            GetMonitorInfoA(*monitor, &mut monitor_info as *mut _ as *mut _);
            monitor_info
        }
    }
}
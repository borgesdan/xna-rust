use crate::xna::csharp::forms::{Screen, SystemInformation};
use crate::xna::csharp::Rectangle;
use std::{mem, ptr};
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{CreateDCW, DeleteDC, GetDeviceCaps, GetMonitorInfoA, BITSPIXEL, HDC, HMONITOR, MONITORINFO, MONITORINFOEXA, PLANES};
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;

impl Screen {
    pub fn from_monitor(hmonitor: HMONITOR, hdc: HDC) -> Self {
        let mut screen_dc = hdc.clone();
        let mut screen = Screen::default();
        let primary_monitor = SystemInformation::primary_monitor();

        if SystemInformation::multi_monitor_support() || hmonitor == primary_monitor {
            screen.bounds = SystemInformation::virtual_screen();
            screen.primary = true;
            screen.device_name = "DISPLAY".to_string();
        } else {
            let info = Self::get_monitor_info(&hmonitor);
            screen.bounds = Rectangle::from_ltrb(
                info.monitorInfo.rcMonitor.left,
                info.monitorInfo.rcMonitor.top,
                info.monitorInfo.rcMonitor.right,
                info.monitorInfo.rcMonitor.bottom
            );
            screen.primary = (info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY) != 0;

            let mut name: [u8; 32] = [0; 32];
            let mut index = 0;
            for i in info.szDevice {
                name[index] = i as u8;
                index += 1;
            }

            screen.device_name = String::from_utf8(name.to_vec()).unwrap();

            if hdc.is_invalid() {
                unsafe {
                    screen_dc = CreateDCW(
                        PCWSTR(info.szDevice.as_ptr() as *const u16),
                        PCWSTR(ptr::null()),
                        PCWSTR(ptr::null()),
                        None);
                }
            }
        }

        screen.platform.h_monitor = hmonitor;

        unsafe {
            screen.bit_depth = GetDeviceCaps(Some(screen_dc), BITSPIXEL);
            screen.bit_depth = screen.bit_depth * GetDeviceCaps(Some(screen_dc), PLANES);

            if hdc != screen_dc {
                DeleteDC(screen_dc);
            }
        }

        screen
    }

    pub fn all_screens() -> Vec<Self> {
        //TODO: get all screens
        let primary_monitor = SystemInformation::primary_monitor();
        let primary_screen = Screen::from_monitor(primary_monitor, HDC::default());
        let mut screens = Vec::<Screen>::new();
        screens.push(primary_screen);

        screens
    }

    fn get_monitor_info(monitor: &HMONITOR) -> MONITORINFOEXA {
        let mut monitor_info: MONITORINFOEXA = unsafe { mem::zeroed() };
        monitor_info.monitorInfo.cbSize = size_of::<MONITORINFOEXA>() as u32;

        let mut p_info_exa: *mut MONITORINFOEXA = &mut monitor_info;
        let p_info = p_info_exa as *mut MONITORINFO;

        unsafe {
            //GetMonitorInfoA(*monitor, &mut monitor_info as *mut _ as *mut _);
            let _ =GetMonitorInfoA(*monitor, p_info);
            monitor_info
        }
    }
}
use crate::xna::csharp::forms::{Screen, SystemInformation};
use crate::xna::csharp::Rectangle;
use std::{mem, ptr};
use windows::core::{BOOL, PCWSTR};
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{CreateDCW, DeleteDC, EnumDisplayMonitors, GetDeviceCaps, GetMonitorInfoA, BITSPIXEL, HDC, HMONITOR, MONITORINFO, MONITORINFOEXA, PLANES};
use windows::Win32::UI::WindowsAndMessaging::MONITORINFOF_PRIMARY;

impl Screen {
    pub fn new(hmonitor: HMONITOR, hdc: HDC) -> Self {
        let mut screen_dc = hdc.clone();
        let mut screen = Screen::default();
        let primary_monitor = SystemInformation::primary_monitor();

        if SystemInformation::multi_monitor_support() || hmonitor == primary_monitor {
            screen.bounds = SystemInformation::virtual_screen();
            screen.primary = true;
            screen.device_name = "\\\\.\\DISPLAY1".to_string();
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
        let mut monitors: Vec<Screen> = Vec::new();
        let monitors_ptr = &mut monitors as *mut _;

        if SystemInformation::multi_monitor_support() {
            unsafe {
                _ = EnumDisplayMonitors(None, None, Some(Self::enumerate_monitors_callback), LPARAM(monitors_ptr as isize));
            }
        } else {
            let primary = SystemInformation::primary_monitor();
            let screen = Screen::new(primary, HDC::default());

            monitors.push(screen);
        }

        monitors
    }

    unsafe extern "system" fn enumerate_monitors_callback(h_monitor: HMONITOR, _: HDC, _: *mut RECT, lparam: LPARAM) -> BOOL {
        let monitors = &mut *(lparam.0 as *mut Vec<Screen>);
        let screen = Screen::new(h_monitor, HDC::default());

        monitors.push(screen);

        BOOL(1)
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
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, HMONITOR, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetSystemMetrics, SM_CMONITORS, SM_CXSCREEN, SM_CXVIRTUALSCREEN, SM_CYSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN};
use crate::xna::csharp::forms::SystemInformation;
use crate::xna::csharp::Rectangle;

impl SystemInformation{
    pub fn multi_monitor_support() -> bool {
        unsafe {
            let metrics = GetSystemMetrics(SM_CMONITORS);
            metrics != 0
        }
    }

    pub fn virtual_screen() -> Rectangle {
        unsafe {
            if Self::multi_monitor_support() {
                let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
                let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
                let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
                let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

                return Rectangle {
                    x, y, width, height
                };
            }

            let size = Self::primary_monitor_size();

            Rectangle {
                x: 0,
                y: 0,
                width: size.cx,
                height: size.cy
            }
        }
    }

    pub fn primary_monitor() -> HMONITOR {
        unsafe {
            let h_desktop = GetDesktopWindow();
            MonitorFromWindow(h_desktop, MONITOR_DEFAULTTOPRIMARY)
        }
    }

    pub fn primary_monitor_size() -> SIZE {
        unsafe {
            let cx = GetSystemMetrics(SM_CXSCREEN);
            let cy = GetSystemMetrics(SM_CYSCREEN);

            SIZE { cx, cy }
        }
    }
}
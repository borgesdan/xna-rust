use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIOutput, DXGI_ENUM_MODES, DXGI_ENUM_MODES_INTERLACED, DXGI_ENUM_MODES_SCALING, DXGI_ENUM_MODES_STEREO};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_UNKNOWN, DXGI_MODE_DESC};
use crate::xna::csharp::Exception;
use crate::xna::framework::game::{GraphicsDeviceManager, GraphicsProfile};
use crate::xna::framework::graphics::{DepthFormat, DisplayMode, DisplayModeCollection, DisplayModeScaling, GraphicsAdapter, GraphicsAdapterOutput, ScanlineOrder, SurfaceFormat};
use crate::xna::framework::Rectangle;
use crate::xna::platform::windows::WindowsGraphicsAdapterOutput;

impl GraphicsAdapter {
    pub fn adapters() -> Result<Vec<GraphicsAdapter>, Exception> {

        let mut adapters : Vec<GraphicsAdapter> = Vec::new();

        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap();
            let mut count: u32 = 0;

            loop {
                let result = Self::create(&factory, count.clone());

                if result.is_err() {
                    if count > 0 {
                        break;
                    } else {
                        let inner = Box::new(result.err().unwrap());
                        return Err(Exception::new("Cannot find graphics adapters", Some(inner)));
                    }
                }

                count += 1;
            }
        }

        Ok(adapters)
    }

    pub fn default_adapter() -> Result<GraphicsAdapter, Exception> {
        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap();

            let result = Self::create(&factory, 0);

            if result.is_err() {
                let inner = Box::new(result.err().unwrap());
                return Err(Exception::new("Cannot find graphics adapters", Some(inner)));
            }

            Ok(result?)
        }
    }

    pub fn is_wide_screen() -> bool {
        true
    }

    fn get_outputs(dx_adapter: &IDXGIAdapter) -> Result<Vec<GraphicsAdapterOutput>, Exception> {
        let mut outputs: Vec<GraphicsAdapterOutput> = Vec::new();

        unsafe {
            for i in 0u32..{
                let output = dx_adapter.EnumOutputs(i);

                if output.is_err(){
                    break;
                }

                let output = output.unwrap();
                let mut description = output.GetDesc();

                if description.is_err() {
                    return Err(Exception::new("Error getting output description", None));
                }

                let description = description.unwrap();
                let device_name = String::from_utf16(&description.DeviceName);

                if device_name.is_err() {
                    return Err(Exception::new("Error converting device name", None));
                }

                let supported_display_modes = Self::get_output_supported_display_modes(&output)?;
                let back_buffer_width = GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH;
                let back_buffer_height = GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT;
                let current_display_mode = Self::get_output_current_display_mode(&supported_display_modes, &SurfaceFormat::Color, back_buffer_width, back_buffer_height);

                let mut out_adapter = GraphicsAdapterOutput {
                    device_name: device_name.unwrap(),
                    attached_to_desktop: description.AttachedToDesktop.as_bool(),
                    desktop_coordinates: Rectangle {
                        x: description.DesktopCoordinates.left,
                        y: description.DesktopCoordinates.top,
                        width: description.DesktopCoordinates.right,
                        height: description.DesktopCoordinates.bottom,
                    },
                    platform: WindowsGraphicsAdapterOutput {
                        output: Some(output)
                    },
                    display_mode_collection: supported_display_modes,

                    current_display_mode: current_display_mode,
                };

                outputs.push(out_adapter);
            }

            Ok(outputs)
        }
    }

    fn get_output_supported_display_modes(output: &IDXGIOutput) -> Result<DisplayModeCollection, Exception> {
        unsafe {
            //TODO: no momento só Color é suportado
            let format = SurfaceFormat::Color.to_dx();

            let mut num_modes = 0;

            let list = output.GetDisplayModeList(format, DXGI_ENUM_MODES_SCALING,
                                                  &mut num_modes, None);

            if list.is_err() {
                return Err(Exception::new("GetDisplayModeList failed.", None));
            }

            list.unwrap();

            let mut display_modes = vec![DXGI_MODE_DESC::default(); num_modes as usize];

            let list = output.GetDisplayModeList(
                format,
                DXGI_ENUM_MODES_SCALING,
                &mut num_modes,
                Some(display_modes.as_mut_ptr()),
            );

            if list.is_err() {
                return Err(Exception::new("GetDisplayModeList failed.", None));
            }

            list.unwrap();

            let mut supported_displays = vec![DisplayMode::default(); num_modes as usize];

            for mode in display_modes {
                let dm = DisplayMode {
                    height: mode.Height,
                    width: mode.Width,
                    format: SurfaceFormat::Color,
                    refresh_rate_denominator: mode.RefreshRate.Denominator,
                    refresh_rate_numerator: mode.RefreshRate.Numerator,
                    scanline_order: ScanlineOrder::from(&mode.ScanlineOrdering),
                    scaling: DisplayModeScaling::from(&mode.Scaling),
                };

                supported_displays.push(dm);
            }

            Ok(DisplayModeCollection { display_modes: supported_displays})
        }
    }

    fn get_output_current_display_mode(collection: &DisplayModeCollection, surface_format: &SurfaceFormat, width: u32, height: u32) -> Option<DisplayMode> {
        let modes = collection.query(surface_format);

        if modes.display_modes.is_empty() {
            return None;
        }

        let mut current_display : Option<DisplayMode> = None;

        for mode in modes.display_modes {
            if mode.width == width && mode.height == height {
                current_display = Some(mode);
                break;
            } else if current_display.is_some() {
                if mode.width <= width && mode.height <= height {
                    current_display = Some(mode);
                }
            } else {
                current_display = Some(mode);
            }
        }

        current_display
    }

    fn create(factory: &IDXGIFactory, index: u32) -> Result<GraphicsAdapter, Exception> {
        unsafe{
            let adapter = factory.EnumAdapters(index);

            if adapter.is_err() {
                return Err(Exception::new("EnumAdapters failed.", None));
            }

            let adapter = adapter.unwrap();
            let description = adapter.GetDesc();

            if description.is_err() {
                return Err(Exception::new("GetDesc failed.", None));
            }

            let description = description.unwrap();

            let mut adp = GraphicsAdapter::default();
            adp.index = index;
            adp.device_id = description.DeviceId;
            adp.is_default = true;
            adp.revision = description.Revision;
            adp.sub_system_id = description.SubSysId;
            adp.vendor_id = description.VendorId;
            adp.description = String::from_utf16(&description.Description).unwrap();
            adp.platform.factory = Some(factory.clone());
            adp.platform.adapter = Some(adapter.clone());

            let outputs = Self::get_outputs(&adapter)?;
            if !outputs.is_empty() {
                adp.current_output = Some(outputs[0].clone());
                adp.outputs = outputs;
            }

            Ok(adp)
        }
    }

    pub fn query_back_buffer_format(&self, format: &SurfaceFormat, depth_format: &DepthFormat, multi_sample_count: i32)
    -> Result<(SurfaceFormat, DepthFormat, u32), Exception> {
        if format.to_dx() == DXGI_FORMAT_UNKNOWN {
            return Err(Exception::new("Unsupported backbuffer format.", None));
        }

        let mut selected_format = *format;
        let selected_depth_format = *depth_format;
        let selected_multi_sample_count = *multi_sample_count;

        let mode_to_match = DXGI_MODE_DESC {
            Format: format.to_dx(),
            ..Default::default()
        };

        let mut closest_match = DXGI_MODE_DESC::default();
        let output = self.current_output
            .as_ref().unwrap()
            .platform.output
            .as_ref().unwrap();

        unsafe {
            let result = output.FindClosestMatchingMode(&mode_to_match, &mut closest_match, None);

            if result.is_err() {
                return Err(Exception::new("FindClosestMatching failed.", None));
            }
        }

        selected_format = SurfaceFormat::from(closest_match.Format);

        Ok((selected_format, selected_depth_format, selected_multi_sample_count))
    }
}


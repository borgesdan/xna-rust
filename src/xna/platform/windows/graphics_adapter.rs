use std::error::Error;
use crate::xna::csharp::Exception;
use crate::xna::framework::game::GraphicsDeviceManager;
use crate::xna::framework::graphics::{DepthFormat, DisplayMode, DisplayModeCollection, DisplayModeScaling, GraphicsAdapter, GraphicsAdapterOutput, ScanlineOrder, SurfaceFormat};
use crate::xna::framework::Rectangle;
use crate::xna::platform::windows::WindowsGraphicsAdapterOutput;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_UNKNOWN, DXGI_MODE_DESC};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIOutput, DXGI_ENUM_MODES_SCALING};
use crate::xna::ExceptionConverter;

impl GraphicsAdapter {
    pub fn adapters() -> Result<Vec<GraphicsAdapter>, Exception> {

        let mut adapters : Vec<GraphicsAdapter> = Vec::new();

        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>()
                .unwrap_or_exception("Failed to create IDXGIFactory")?;

            let mut count: u32 = 0;

            loop {
                let result = Self::create(&factory, count.clone())?;

                if result.is_none() {
                    break;
                }

                adapters.push(result.unwrap());
                count += 1;
            }
        }

        Ok(adapters)
    }

    pub fn default_adapter() -> Result<GraphicsAdapter, Exception> {
        unsafe {
            let factory = CreateDXGIFactory::<IDXGIFactory>()
                .unwrap_or_exception("Failed to create DXGIFactory")?;

            let result = Self::create(&factory, 0)?;

            if result.is_none() {
                return Err(Exception::new("Default adapter not find.", None));
            }

            Ok(result.unwrap())
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

                let output = output?;
                let mut description = output.GetDesc()
                    .unwrap_or_exception("Error getting output description")?;

                let device_nam_u16 = String::from_utf16(&description.DeviceName);

                if device_nam_u16.is_err() {
                    return Err(Exception::new("Error converting device name", None));
                }

                let device_name_16 = device_nam_u16.unwrap();
                let terminator = device_name_16.find('\0');

                let slice = std::slice::from_raw_parts(device_name_16.as_ptr(), terminator.unwrap());
                let device_name = String::from_utf8(slice.to_vec()).unwrap();

                let supported_display_modes = Self::get_output_supported_display_modes(&output)?;
                let back_buffer_width = GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH;
                let back_buffer_height = GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT;
                let current_display_mode = Self::get_output_current_display_mode(&supported_display_modes, &SurfaceFormat::Color, back_buffer_width, back_buffer_height);

                let out_adapter = GraphicsAdapterOutput {
                    device_name,
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

                    current_display_mode,
                };

                outputs.push(out_adapter);
            }

            Ok(outputs)
        }
    }

    fn get_output_supported_display_modes(output: &IDXGIOutput) -> Result<DisplayModeCollection, Exception> {
        unsafe {
            //TODO: no momento só Color é suportado
            let format = DXGI_FORMAT::from(SurfaceFormat::Color);

            let mut num_modes = 0;

            let list = output.GetDisplayModeList(format, DXGI_ENUM_MODES_SCALING,
                                                  &mut num_modes, None).unwrap_or_exception("GetDisplayModeList failed.")?;

            let mut display_modes = vec![DXGI_MODE_DESC::default(); num_modes as usize];

            output.GetDisplayModeList(
                format,
                DXGI_ENUM_MODES_SCALING,
                &mut num_modes,
                Some(display_modes.as_mut_ptr()),
            ).unwrap_or_exception("GetDisplayModeList failed.")?;

            let mut supported_displays = vec![DisplayMode::default(); num_modes as usize];

            for mode in display_modes {
                let dm = DisplayMode {
                    height: mode.Height,
                    width: mode.Width,
                    format: SurfaceFormat::Color,
                    refresh_rate_denominator: mode.RefreshRate.Denominator,
                    refresh_rate_numerator: mode.RefreshRate.Numerator,
                    scanline_order: ScanlineOrder::from(mode.ScanlineOrdering.clone()),
                    scaling: DisplayModeScaling::from(mode.Scaling.clone()),
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

    fn create(factory: &IDXGIFactory, index: u32) -> Result<Option<GraphicsAdapter>, Exception> {
        unsafe{
            let adapter = factory.EnumAdapters(index);

            if adapter.is_err() {
                return Ok(None);
            }

            let adapter = adapter?;
            let description = adapter.GetDesc()
                .unwrap_or_exception("GetDesc failed")?;

            let mut adp = GraphicsAdapter::default();
            adp.index = index;
            adp.device_id = description.DeviceId;
            adp.is_default = index == 0;
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

            Ok(Some(adp))
        }
    }

    pub fn query_back_buffer_format(&self, format: &SurfaceFormat, depth_format: &DepthFormat, multi_sample_count: u32)
    -> Result<(SurfaceFormat, DepthFormat, u32), Exception> {
        if DXGI_FORMAT::from(format.clone()) == DXGI_FORMAT_UNKNOWN {
            return Err(Exception::new("Unsupported backbuffer format.", None));
        }

        let mut selected_format = *format;
        let selected_depth_format = *depth_format;
        let selected_multi_sample_count = multi_sample_count;

        let mode_to_match = DXGI_MODE_DESC {
            Format: DXGI_FORMAT::from(format.clone()),
            ..Default::default()
        };

        let mut closest_match = DXGI_MODE_DESC::default();
        let output = self.current_output
            .as_ref().unwrap()
            .platform.output
            .as_ref().unwrap();

        unsafe {
            output.FindClosestMatchingMode(&mode_to_match, &mut closest_match, None)
                .unwrap_or_exception("FindClosestMatching failed.")?;
        }

        selected_format = SurfaceFormat::from(closest_match.Format);

        Ok((selected_format, selected_depth_format, selected_multi_sample_count))
    }
}


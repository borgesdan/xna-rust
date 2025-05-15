use windows::core::Interface;
use crate::xna::csharp::Exception;
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice, IPackedVector, PresentInterval, PresentationParameters, RenderTarget2D};
use crate::xna::framework::Color;
use crate::xna::platform::windows::WindowsGraphicsDevice;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext, ID3D11RasterizerState, ID3D11SamplerState, D3D11_BLEND_DESC, D3D11_CREATE_DEVICE_DEBUG, D3D11_DEPTH_STENCIL_DESC, D3D11_RASTERIZER_DESC, D3D11_SAMPLER_DESC, D3D11_SDK_VERSION, D3D11_VIEWPORT};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, DXGI_MWA_FLAGS, DXGI_PRESENT};
use crate::xna::{ExceptionConverter, SilentExceptionConverter};

impl GraphicsDevice {
    pub fn initialize(&mut self, adapter: Option<GraphicsAdapter>) -> Result<(), Exception> {
        if !self.platform.is_initialized {
            self.create(adapter)?
        }

        unsafe {
            let factory = self.platform.factory.as_ref().unwrap();
            let context = self.platform.context.as_ref().unwrap();
            let device = self.platform.device.as_ref().unwrap();

            //Window association
            factory.MakeWindowAssociation(self.presentation_parameters.platform.hwnd.clone(), DXGI_MWA_FLAGS::default())
                .unwrap_or_exception("MakeWindowAssociation failed")?;

            // Viewport
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: self.presentation_parameters.back_buffer_width as f32,
                Height: self.presentation_parameters.back_buffer_height as f32,
                MaxDepth: 1.0,
                MinDepth: 0.0,
            }];

            context.RSSetViewports(Some(&viewport));

            // States
            self.apply_blend_state()?;
            self.apply_rasterizer_state()?;
            self.apply_sampler_states()?;

            //Swap Chain
            let swap_chain = self.swap_chain.initialize(self)?;
            self.platform.swap_chain = swap_chain;

            //Render Target
            let mut render_target = RenderTarget2D::from_back_buffer(self)?;
            render_target.initialize(self)?;

            let render_views = [render_target.platform.view.clone()];

            self.platform.context.as_ref().unwrap().OMSetRenderTargets(Some(&render_views), None);

            self.platform.render_target = render_target.platform.view;

            Ok(())
        }
    }
    fn create(&mut self, adapter: Option<GraphicsAdapter>) -> Result<(), Exception> {
        unsafe {
            let flags = D3D11_CREATE_DEVICE_DEBUG;

            let hmodule = HMODULE::default();
            let factory = CreateDXGIFactory::<IDXGIFactory>().unwrap_or_exception("")?;

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

            let dx_adapter :Option<&IDXGIAdapter> = if adapter.is_some() { adapter.as_ref().unwrap().platform.adapter.as_ref() } else { None };

            D3D11CreateDevice(
                dx_adapter,
                D3D_DRIVER_TYPE_HARDWARE,
                hmodule,
                flags,
                Some(&feature_levels),
                D3D11_SDK_VERSION,
                Some(&mut device),
                Some(&mut feature_level),
                Some(&mut context),
            ).unwrap_or_exception("Error creating DXGI adapter")?;

            let graphics_adapter = if adapter.is_some() { adapter } else { Some(GraphicsAdapter::default_adapter()?) };
            self.adapter = graphics_adapter;
            self.platform.context = context;
            self.platform.device = device;
            self.platform.factory = Some(factory);
            self.platform.feature_level = feature_level;

            Ok(())
        }
    }

    pub fn present(&self) -> Result<(), Exception> {
        unsafe {
            // Presentation
            let vsync: u32;

            match self.presentation_parameters.presentation_interval {
                PresentInterval::Default => vsync = 1,
                PresentInterval::One => vsync = 1,
                PresentInterval::Two  => vsync = 2,
                PresentInterval::Immediate => vsync = 0,
            }

            self.platform.swap_chain
                .unwrap_ref_or_default_exception()?
                .Present(vsync, DXGI_PRESENT::default())
                .unwrap();

            let view = self.platform.render_target
                .unwrap_ref_or_default_exception()?
                .clone();

            let render_views = [Some(view.clone())];

            self.platform.context
                .unwrap_ref_or_default_exception()?
                .OMSetRenderTargets(Some(&render_views), None);

            Ok(())
        }
    }

    pub fn clear(&self, color: Color) -> Result<(), Exception> {
        let rgba = color.to_vector4();

        let background = [rgba.x, rgba.y, rgba.z, rgba.w];

        let render_target_view = self.platform.render_target
            .unwrap_ref_or_default_exception()?.clone();

        unsafe {
            self.platform.context.unwrap_ref_or_default_exception()?
                .ClearRenderTargetView(&render_target_view, &background);
        }

        Ok(())
    }

    fn apply_sampler_states(&mut self) -> Result<(), Exception> {
        let collection = &self.sampler_state_collection;

        if collection.samplers.is_empty() {
            return Ok(());
        }

        let device = self.platform.device.unwrap_ref_or_default_exception()?;
        let context = self.platform.context.unwrap_ref_or_default_exception()?;
        let mut samplers: Vec<Option<ID3D11SamplerState>> = Vec::new();

        unsafe {
            for sampler in &collection.samplers {
                let description = D3D11_SAMPLER_DESC::from(sampler.clone());
                let mut dx_sampler: Option<ID3D11SamplerState> = None;

                device.CreateSamplerState(&description, Some(&mut dx_sampler))
                    .unwrap_or_exception("Error creating DXGI sampler")?;
            }

            context.PSSetSamplers(0, Some(samplers.as_slice()));
        }

        Ok(())
    }

    fn apply_depth_stencil_state(&mut self)-> Result<(), Exception> {
        let description =  D3D11_DEPTH_STENCIL_DESC::from(self.depth_stencil_state);
        let device = self.platform.device.unwrap_ref_or_default_exception()?;
        let context = self.platform.context.unwrap_ref_or_default_exception()?;
        let mut dx_depth: Option<ID3D11DepthStencilState> = None;

        unsafe {
            device.CreateDepthStencilState(&description, Some(&mut dx_depth))
                .unwrap_or_exception("Error creating DXGI depth state")?;

            context.OMSetDepthStencilState(dx_depth.as_ref(), 0);

            self.platform.depth_stencil_state = dx_depth;
        }

        Ok(())
    }

    fn apply_rasterizer_state(&mut self)-> Result<(), Exception> {
        //Convert
        let description = D3D11_RASTERIZER_DESC::from(self.rasterizer_state);
        let device = self.platform.device.unwrap_ref_or_default_exception()?;
        let context = self.platform.context.unwrap_ref_or_default_exception()?;
        let mut dx_rasterizer: Option<ID3D11RasterizerState> = None;

        unsafe {
            device.CreateRasterizerState(&description, Some(&mut dx_rasterizer))
                .unwrap_ref_or_exception("Error creating DXGI rasterizer state")?;

            context.RSSetState(dx_rasterizer.as_ref());

            self.platform.rasterizer_state = dx_rasterizer;
        }

        Ok(())
    }

    fn apply_blend_state(&mut self)-> Result<(), Exception> {
        let description = D3D11_BLEND_DESC::from(self.blend_state);
        let device = self.platform.device.unwrap_ref_or_default_exception()?;
        let context = self.platform.context.unwrap_ref_or_default_exception()?;
        let mut dx_blend_state: Option<ID3D11BlendState> = None;

        unsafe {
            device.CreateBlendState(&description, Some(&mut dx_blend_state))
                .unwrap_ref_or_exception("Error creating DXGI blend state")?;

            let blend_factor = self.blend_state.blend_factor.to_vector4();
            let factor = [blend_factor.x, blend_factor.y, blend_factor.z, blend_factor.w];
            let sample_mask = self.blend_state.multi_sample_mask;

            context.OMSetBlendState(dx_blend_state.as_ref(), Some(&factor), sample_mask);

            self.platform.blend_state = dx_blend_state;
        }

        Ok(())
    }

    pub fn reset(&mut self, parameters: &PresentationParameters, adapter: &GraphicsAdapter) -> Result<(), Exception> {
        self.adapter = Some(adapter.clone());
        self.presentation_parameters = parameters.clone();
        self.platform = WindowsGraphicsDevice::default();

        Ok(())
    }
}
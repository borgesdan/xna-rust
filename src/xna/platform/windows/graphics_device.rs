use crate::xna::csharp::Exception;
use crate::xna::framework::graphics::{GraphicsDevice, IPackedVector, PresentInterval};
use crate::xna::framework::Color;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext, ID3D11RasterizerState, ID3D11SamplerState, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11_VIEWPORT};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIFactory, DXGI_MWA_FLAGS, DXGI_PRESENT};

impl GraphicsDevice {
    fn create(&mut self) -> Result<(), Exception> {
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
                Some(&mut context),
            ).unwrap();

            self.platform.context = context;
            self.platform.device = device;
            self.platform.factory = Some(factory);
            self.platform.feature_level = feature_level;

            Ok(())
        }
    }

    pub fn initialize(&mut self) -> Result<(), Exception> {
        if !self.platform.is_initialized {
            self.create()?
        }

        unsafe {
            let factory = self.platform.factory.as_ref().unwrap();
            let context = self.platform.context.as_ref().unwrap();
            let device = self.platform.device.as_ref().unwrap();

            //Window association
            factory.MakeWindowAssociation(self.presentation_parameters.platform.hwnd, DXGI_MWA_FLAGS::default())
                .unwrap();

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
            self.apply_blend_state();
            self.apply_rasterizer_state();
            self.apply_sampler_states();

            // Presentation
            let vsync: i32;

            match self.presentation_parameters.presentation_interval {
                PresentInterval::Default => vsync = 1,
                PresentInterval::One => vsync = 1,
                PresentInterval::Two  => vsync = 2,
                PresentInterval::Immediate => vsync = 0,
            }

            //Swap Chain
            let swap_chain = self.swap_chain.initialize(self);
            self.platform.swap_chain = swap_chain;

            //Render Target
            let mut render_target = self.render_target.from_back_buffer(self);
            render_target.initialize(self);

            let render_views = [render_target.platform.view.clone()];

            self.platform.context.as_ref().unwrap().OMSetRenderTargets(Some(&render_views), None);

            self.platform.render_target = render_target.platform.view;

            Ok(())
        }
    }

    pub fn present(&self) {
        unsafe {
            self.platform.swap_chain.as_ref().unwrap().Present(1, DXGI_PRESENT::default()).unwrap();

            let view = self.render_target.platform.view.as_ref().unwrap().clone();

            let render_views = [Some(view.clone())];

            self.platform.context.as_ref().unwrap().OMSetRenderTargets(Some(&render_views), None);
        }
    }

    pub fn clear(&self, color: &Color) {
        let rgba = color.to_vector4();

        let background = [rgba.x, rgba.y, rgba.z, rgba.w];

        let render_target_view = self.platform.render_target.as_ref().unwrap();

        unsafe {
            self.platform.context.as_ref().unwrap()
                .ClearRenderTargetView(render_target_view, &background);
        }
    }

    fn apply_sampler_states(&mut self) {
        let collection = &self.sampler_state_collection;

        if collection.samplers.is_empty() {
            return;
        }

        let device = self.platform.device.as_ref().unwrap();
        let context = self.platform.context.as_ref().unwrap();
        let mut samplers: Vec<Option<ID3D11SamplerState>> = Vec::new();

        unsafe {
            for sampler in &collection.samplers {
                let description = sampler.to_dx();
                let mut dx_sampler: Option<ID3D11SamplerState> = None;

                device.CreateSamplerState(&description, Some(&mut dx_sampler)).unwrap();
            }

            context.PSSetSamplers(0, Some(samplers.as_slice()));
        }
    }

    fn apply_depth_stencil_state(&mut self) {
        let description =  self.depth_stencil_state.to_dx();
        let device = self.platform.device.as_ref().unwrap();
        let context = self.platform.context.as_ref().unwrap();
        let mut dx_depth: Option<ID3D11DepthStencilState> = None;

        unsafe {
            device.CreateDepthStencilState(&description, Some(&mut dx_depth))
                .unwrap();

            context.OMSetDepthStencilState(dx_depth.as_ref(), 0);

            self.platform.depth_stencil_state = dx_depth;
        }
    }

    fn apply_rasterizer_state(&mut self) {
        //Convert
        let description = self.rasterizer_state.to_dx();
        let device = self.platform.device.as_ref().unwrap();
        let context = self.platform.context.as_ref().unwrap();
        let mut dx_rasterizer: Option<ID3D11RasterizerState> = None;

        unsafe {
            device.CreateRasterizerState(&description, Some(&mut dx_rasterizer))
                .unwrap();

            context.RSSetState(dx_rasterizer.as_ref());

            self.platform.rasterizer_state = dx_rasterizer;
        }
    }

    fn apply_blend_state(&mut self) {
        let description = self.blend_state.to_dx();
        let device = self.platform.device.as_ref().unwrap();
        let context = self.platform.context.as_ref().unwrap();
        let mut dx_blend_state: Option<ID3D11BlendState> = None;

        unsafe {
            device.CreateBlendState(&description, Some(&mut dx_blend_state))
                .unwrap();

            let blend_factor = self.blend_state.blend_factor.to_vector4();
            let factor = [blend_factor.x, blend_factor.y, blend_factor.z, blend_factor.w];
            let sample_mask = self.blend_state.multi_sample_mask;

            context.OMSetBlendState(dx_blend_state.as_ref(), Some(&factor), sample_mask);

            self.platform.blend_state = dx_blend_state;
        }
    }
}
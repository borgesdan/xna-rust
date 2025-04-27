use windows::core::BOOL;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext, ID3D11RasterizerState, ID3D11RenderTargetView, ID3D11SamplerState, D3D11_BLEND_DESC, D3D11_COMPARISON_LESS_EQUAL, D3D11_CREATE_DEVICE_DEBUG, D3D11_CULL_BACK, D3D11_DEPTH_STENCIL_DESC, D3D11_FILL_SOLID, D3D11_RASTERIZER_DESC, D3D11_SAMPLER_DESC, D3D11_SDK_VERSION, D3D11_VIEWPORT};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIFactory, IDXGISwapChain, DXGI_MWA_FLAGS, DXGI_PRESENT};
use crate::xna::framework::Color;
use crate::xna::framework::graphics::{GraphicsDevice, IPackedVector, PresentInterval, PresentationParameters};
use crate::xna::platform::windows::bool_to_win_bool;
use crate::xna::platform::windows::render_target_2d::WindowsRenderTarget2D;

#[derive(Default)]
pub struct WindowsGraphicsDevice {
    pub device: Option<ID3D11Device>,
    pub context: Option<ID3D11DeviceContext>,
    pub factory: Option<IDXGIFactory>,
    pub feature_level: D3D_FEATURE_LEVEL,
    pub blend_state: Option<ID3D11BlendState>,
    pub rasterizer_state: Option<ID3D11RasterizerState>,
    pub swap_chain: Option<IDXGISwapChain>,
    pub depth_stencil_state: Option<ID3D11DepthStencilState>,
    pub render_target: Option<WindowsRenderTarget2D>,
    pub sampler_state_collection: Vec<Option<ID3D11SamplerState>>,
    pub background_color: Color,
    pub parameters: WindowsPresentationParameters,
    pub base: GraphicsDevice,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct WindowsPresentationParameters {
    pub hwnd: HWND,
    pub base: PresentationParameters,
}

impl GraphicsDevice {
    pub fn create(&self) -> WindowsGraphicsDevice {
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

            WindowsGraphicsDevice {
                context: context,
                device: device,
                factory: Some(factory),
                feature_level: feature_level,
                background_color: Color::cornflower_blue(),
                base: self.clone(),
                ..Default::default()
            }
        }
    }
}

impl WindowsGraphicsDevice {
    pub fn initialize(&mut self) {
        self.parameters = self.parameters;

        unsafe {
            let factory = self.factory.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();
            let device = self.device.as_ref().unwrap();

            //Window association
            factory.MakeWindowAssociation(self.parameters.hwnd, DXGI_MWA_FLAGS::default())
                .unwrap();

            // Viewport
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: self.parameters.base.back_buffer_width as f32,
                Height: self.parameters.base.back_buffer_height as f32,
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

            match self.parameters.base.presentation_interval {
                PresentInterval::Default => vsync = 1,
                PresentInterval::One => vsync = 1,
                PresentInterval::Two  => vsync = 2,
                PresentInterval::Immediate => vsync = 0,
            }

            //Swap Chain
            let swap_chain = self.base.swap_chain.initialize(self);
            self.swap_chain = swap_chain;

            //Render Target
            let mut render_target = self.base.render_target.from_back_buffer(self);
            render_target.initialize(self);

            let render_views = [render_target.view.clone()];

            self.context.as_ref().unwrap().OMSetRenderTargets(Some(&render_views), None);

            self.render_target = Some(render_target);
        }
    }

    pub fn present(&self) {
        unsafe {
            self.swap_chain.as_ref().unwrap()
                .Present(1, DXGI_PRESENT::default()).unwrap();

            let view = self.render_target.as_ref().unwrap().view.clone();
            let render_views = [view];
            self.context.as_ref().unwrap().OMSetRenderTargets(Some(&render_views), None);
        }
    }

    pub fn clear(&self, color: &Color) {
        let rgba = color.to_vector4();

        let background = [rgba.x, rgba.y, rgba.z, rgba.w];

        let render_target_view = self.render_target.as_ref().unwrap().view.clone();

        unsafe {
            self.context.as_ref().unwrap()
                .ClearRenderTargetView(render_target_view.as_ref(), &background);
        }

    }

    fn apply_sampler_states(&mut self) {
        let collection = &self.base.sampler_state_collection;

        if collection.samplers.is_empty() {
            return;
        }

        let device = self.device.as_ref().unwrap();
        let context = self.context.as_ref().unwrap();
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
        let description =  self.base.depth_stencil_state.to_dx();
        let device = self.device.as_ref().unwrap();
        let context = self.context.as_ref().unwrap();
        let mut dx_depth: Option<ID3D11DepthStencilState> = None;

        unsafe {
            device.CreateDepthStencilState(&description, Some(&mut dx_depth))
                .unwrap();

            context.OMSetDepthStencilState(dx_depth.as_ref(), 0);

            self.depth_stencil_state = dx_depth;
        }
    }

    fn apply_rasterizer_state(&mut self) {
        //Convert
        let description = self.base.rasterizer_state.to_dx();
        let device = self.device.as_ref().unwrap();
        let context = self.context.as_ref().unwrap();
        let mut dx_rasterizer: Option<ID3D11RasterizerState> = None;

        unsafe {
            device.CreateRasterizerState(&description, Some(&mut dx_rasterizer))
                .unwrap();

            context.RSSetState(dx_rasterizer.as_ref());

            self.rasterizer_state = dx_rasterizer;
        }
    }

    fn apply_blend_state(&mut self) {
        let description = self.base.blend_state.to_dx();
        let device = self.device.as_ref().unwrap();
        let context = self.context.as_ref().unwrap();
        let mut dx_blend_state: Option<ID3D11BlendState> = None;

        unsafe {
            device.CreateBlendState(&description, Some(&mut dx_blend_state))
                .unwrap();

            let blend_factor = self.base.blend_state.blend_factor.to_vector4();
            let factor = [blend_factor.x, blend_factor.y, blend_factor.z, blend_factor.w];
            let sample_mask = self.base.blend_state.multi_sample_mask;

            context.OMSetBlendState(dx_blend_state.as_ref(), Some(&factor), sample_mask);

            self.blend_state = dx_blend_state;
        }
    }

}
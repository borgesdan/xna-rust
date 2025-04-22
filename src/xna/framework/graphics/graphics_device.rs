use std::ptr;
use std::boxed;
use windows::core::{Param, BOOL};
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct3D;
use windows::Win32::Graphics::Direct3D11::{D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext, ID3D11RasterizerState, ID3D11SamplerState, D3D11_BLEND, D3D11_BLEND_DESC, D3D11_BLEND_ONE, D3D11_BLEND_OP_ADD, D3D11_COMPARISON_LESS_EQUAL, D3D11_CREATE_DEVICE_DEBUG, D3D11_CREATE_DEVICE_FLAG, D3D11_CULL_BACK, D3D11_CULL_MODE, D3D11_DEPTH_STENCIL_DESC, D3D11_FILL_SOLID, D3D11_RASTERIZER_DESC, D3D11_RENDER_TARGET_BLEND_DESC, D3D11_SAMPLER_DESC, D3D11_SDK_VERSION, D3D11_VIEWPORT};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_9_1, D3D_FEATURE_LEVEL_9_2, D3D_FEATURE_LEVEL_9_3};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory, IDXGIAdapter, IDXGIFactory, IDXGIFactory1, DXGI_MWA_FLAGS};
use crate::xna::framework::Color;
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice, IPackedVector, PresentationParameters, SamplerState, SamplerStateCollection};
use crate::xna::Platform::windows::{blend_operation_to_d3dx_blend_op, blend_to_d3dx_blend, bool_to_win_bool, color_write_channels_to_d3dx_color_write_enable};

#[derive(Default)]
pub struct WindowsGraphicsDevice {
    pub device: Option<ID3D11Device>,
    pub context: Option<ID3D11DeviceContext>,
    pub factory: Option<IDXGIFactory>,
    pub feature_level: D3D_FEATURE_LEVEL,
    pub blend_state: Option<ID3D11BlendState>,
    pub rasterizer_state: Option<ID3D11RasterizerState>,
    pub depth_stencil_state: Option<ID3D11DepthStencilState>,
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
    pub fn create() -> WindowsGraphicsDevice {
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
                base: GraphicsDevice::default(),
                ..Default::default()
            }
        }
    }
}

impl WindowsGraphicsDevice {
    pub fn initialize(&mut self, parameters: &WindowsPresentationParameters) {
        self.parameters = self.parameters;

        unsafe {
            let factory = self.factory.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();
            let device = self.device.as_ref().unwrap();

            //Window association
            factory.MakeWindowAssociation(parameters.hwnd, DXGI_MWA_FLAGS::default())
                .unwrap();

            // Viewport
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: parameters.base.back_buffer_width as f32,
                Height: parameters.base.back_buffer_height as f32,
                MaxDepth: 1.0,
                MinDepth: 0.0,
            }];

            context.RSSetViewports(Some(&viewport));

            // States
            self.apply_blend_state();
            self.apply_rasterizer_state();
        }
    }

    fn apply_sampler_states(&mut self) {
        let collection = &self.base.sampler_state_collection;

        if collection.samplers.is_empty() {
            return;
        }

        unsafe {
            let device = self.device.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();
            let mut samplers: Vec<Option<ID3D11SamplerState>> = Vec::new();


            for i in 0..collection.samplers.len() {
                let current = &collection.samplers[i];

                let description = D3D11_SAMPLER_DESC::default();
                let mut dx_sampler: Option<ID3D11SamplerState> = None;

                //Initialize
                device.CreateSamplerState(&description, Some(&mut dx_sampler)).unwrap();
            }

            //Apply
            context.PSSetSamplers(0, Some(samplers.as_slice()));
        }
    }

    fn apply_depth_stencil_state(&mut self) {
        let depth_stencil = &self.base.depth_stencil_state;

        //TODO implementar depth stencil state

        let description = D3D11_DEPTH_STENCIL_DESC {
            DepthFunc: D3D11_COMPARISON_LESS_EQUAL,
            ..Default::default()
        };

        unsafe {
            let device = self.device.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();

            let mut dx_depth: Option<ID3D11DepthStencilState> = None;

            // Initialize
            device.CreateDepthStencilState(&description, Some(&mut dx_depth))
                .unwrap();

            // Apply
            context.OMSetDepthStencilState(dx_depth.as_ref(), 0);

            self.depth_stencil_state = dx_depth;
        }
    }

    fn apply_rasterizer_state(&mut self) {
        let rasterizer = &self.base.rasterizer_state;

        //Convert

        //TODO implementar rasterizer state

        let description = D3D11_RASTERIZER_DESC {
            CullMode: D3D11_CULL_BACK,
            FillMode: D3D11_FILL_SOLID,
            MultisampleEnable: BOOL(1),
            DepthBias: 0,
            SlopeScaledDepthBias: 0.0,
            ScissorEnable: BOOL(0),
            AntialiasedLineEnable: BOOL(1),
            ..Default::default()
        };

        unsafe {
            let device = self.device.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();

            let mut dx_rasterizer: Option<ID3D11RasterizerState> = None;
            // Initialize

            device.CreateRasterizerState(&description, Some(&mut dx_rasterizer))
                .unwrap();

            // Apply
            context.RSSetState(dx_rasterizer.as_ref());

            self.rasterizer_state = dx_rasterizer;
        }
    }

    fn apply_blend_state(&mut self) {
        let blend_state = &self.base.blend_state;

        //Convert

        let mut description = D3D11_BLEND_DESC::default();
        description.AlphaToCoverageEnable = bool_to_win_bool(blend_state.alpha_to_coverage_enable);
        description.IndependentBlendEnable = bool_to_win_bool(blend_state.independent_blend_enable);

        let mut index = 0;
        for target in &blend_state.render_targets {
            description.RenderTarget[index].BlendEnable = bool_to_win_bool(target.enabled);
            description.RenderTarget[index].SrcBlend = blend_to_d3dx_blend(&target.source);
            description.RenderTarget[index].DestBlend = blend_to_d3dx_blend(&target.destination);;
            description.RenderTarget[index].BlendOp = blend_operation_to_d3dx_blend_op(&target.operation);
            description.RenderTarget[index].SrcBlendAlpha = blend_to_d3dx_blend(&target.source_alpha);
            description.RenderTarget[index].DestBlendAlpha = blend_to_d3dx_blend(&target.destination_alpha);
            description.RenderTarget[index].BlendOpAlpha = blend_operation_to_d3dx_blend_op(&target.operation_alpha);
            description.RenderTarget[index].RenderTargetWriteMask = color_write_channels_to_d3dx_color_write_enable(&target.write_mask).0 as u8;

            index += 1;
        }

        unsafe {
            let mut dx_blend_state: Option<ID3D11BlendState> = None;

            // Initialize
            let device = self.device.as_ref().unwrap();
            let context = self.context.as_ref().unwrap();

            device.CreateBlendState(&description, Some(&mut dx_blend_state))
                .unwrap();

            let blend_factor = blend_state.blend_factor.to_vector4();

            let factor = [blend_factor.x, blend_factor.y, blend_factor.z, blend_factor.w];
            let sample_mask = blend_state.multi_sample_mask;

            // Apply
            context.OMSetBlendState(dx_blend_state.as_ref(), Some(&factor), sample_mask);

            self.blend_state = dx_blend_state;
        }
    }
}

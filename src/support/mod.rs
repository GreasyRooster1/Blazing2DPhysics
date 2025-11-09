use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::winit::dpi::LogicalSize;
use imgui_winit_support::winit::event::{Event, WindowEvent};
use imgui_winit_support::winit::event_loop::EventLoop;
use imgui_winit_support::winit::window::WindowAttributes;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;
use crate::sim_state_mutex;
use crate::simulation::SimState;

pub(crate) mod clipboard;

pub const FONT_SIZE: f32 = 18.0;

/// Creates the imgui context
pub fn create_context() -> imgui::Context {
    let mut imgui = Context::create();
    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    imgui.fonts().add_font(&[
        FontSource::TtfData {
            data: include_bytes!("../../resources/JetBrainsMono-VariableFont_wght.ttf"),
            size_pixels: FONT_SIZE,
            config: Some(FontConfig {
                // As imgui-glium-renderer isn't gamma-correct with
                // it's font rendering, we apply an arbitrary
                // multiplier to make the font a bit "heavier". With
                // default imgui-glow-renderer this is unnecessary.
                rasterizer_multiply: 1.5,
                // Oversampling font helps improve text rendering at
                // expense of larger font atlas texture.
                oversample_h: 4,
                oversample_v: 4,
                ..FontConfig::default()
            }),
        }
    ]);
    imgui.set_ini_filename(None);

    imgui
}
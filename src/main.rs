#![feature(random)]

mod support;
mod simulation;
mod ui;

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use glium::{implement_vertex, uniform, Display, Frame, Program, Surface};
use glium::glutin::surface::WindowSurface;
use glium::index::{IndicesSource, NoIndices};
use glium::winit::dpi::LogicalSize;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::EventLoop;
use glium::winit::window::WindowAttributes;
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use lazy_static::lazy_static;
use crate::simulation::{simulation_step, Object, SimState};
use crate::support::{clipboard, create_context};
use crate::ui::render_ui;

lazy_static! {
    static ref sim_state_mutex: Mutex<SimState> = Mutex::new(SimState::start());
}

pub const TARGET_FPS: u64 = 60;
pub const TARGET_SIM_UPDATES: u64 = 100;

pub const GRAVITY: f32 = -9.81;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {


    thread::spawn(move || {
        let frame_duration = Duration::from_secs(1) / TARGET_SIM_UPDATES as u32;

        let mut last_frame_time = Instant::now();
        loop {
            let mut sim_state_guard = sim_state_mutex.lock().unwrap();
            let sim_state = &mut *sim_state_guard;

            simulation_step(sim_state);
            let elapsed_calc = last_frame_time.elapsed();
            sim_state.timings.sim_calc_elapsed = elapsed_calc.as_nanos() as u64;

            drop(sim_state_guard);

            if elapsed_calc < frame_duration {
                thread::sleep(frame_duration - elapsed_calc);
            }


            let mut sim_state_guard = sim_state_mutex.lock().unwrap();
            let sim_state = &mut *sim_state_guard;
            let elapsed_full = last_frame_time.elapsed();
            sim_state.timings.sim_full_elapsed = elapsed_full.as_nanos() as u64;
            drop(sim_state_guard);

            last_frame_time = Instant::now();
        }
    });
    start("Sim")
}

pub fn start(title:&str) {
    let mut imgui = create_context();

    let title = match Path::new(&title).file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => title,
    };
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");

    let window_attributes = WindowAttributes::default()
        .with_title(title)
        .with_inner_size(LogicalSize::new(768, 768));
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);
    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    if let Some(backend) = clipboard::init() {
        imgui.set_clipboard_backend(backend);
    } else {
        eprintln!("Failed to initialize clipboard");
    }

    let mut platform = WinitPlatform::new(&mut imgui);
    {
        let dpi_mode = if let Ok(factor) = std::env::var("IMGUI_EXAMPLE_FORCE_DPI_FACTOR") {
            // Allow forcing of HiDPI factor for debugging purposes
            match factor.parse::<f64>() {
                Ok(f) => HiDpiMode::Locked(f),
                Err(e) => panic!("Invalid scaling factor: {}", e),
            }
        } else {
            HiDpiMode::Default
        };

        platform.attach_window(imgui.io_mut(), &window, dpi_mode);
    }

    let mut last_frame = Instant::now();

    let circle_shader = Program::from_source(&display, include_str!("../resources/circle.vert"), include_str!("../resources/circle.frag"), Some(include_str!("../resources/circle.geom"))).unwrap();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let start_time = Instant::now();
                let ui = imgui.frame();
                let mut run = true;

                if !run {
                    window_target.exit();
                }

                let sim_state_guard = sim_state_mutex.lock().expect("Failed to lock sim_state");
                let mut sim_state = (*sim_state_guard).clone();
                drop(sim_state_guard);

                let mut target = display.draw();
                target.clear_color(0.0,0.0,0.0, 1.0);

                render(&mut target, &display, &mut sim_state,&circle_shader);

                let frame_times= get_elapsed(start_time);

                let mut sim_state_guard = sim_state_mutex.lock().expect("Failed to lock sim_state");
                (*sim_state_guard).timings.frame_calc_elapsed = frame_times.0;
                (*sim_state_guard).timings.frame_full_elapsed = frame_times.1;
                drop(sim_state_guard);


                render_ui(ui,&mut sim_state);

                platform.prepare_render(ui, &window);
                let draw_data = imgui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::AboutToWait => {
                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                if new_size.width > 0 && new_size.height > 0 {
                    display.resize((new_size.width, new_size.height));
                }
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            event => {
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        })
        .expect("EventLoop error");
}

pub fn get_elapsed(start_time: Instant) -> (u64, u64) {
    let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as u64;

    let wait_millis = match 1_000_000_000 / TARGET_FPS >= elapsed_time {
        true => 1_000_000_000 / TARGET_FPS - elapsed_time,
        false => 0
    };
    thread::sleep(Duration::from_nanos(wait_millis));
    let elapsed_total = Instant::now().duration_since(start_time).as_nanos() as u64;
    (elapsed_time,elapsed_total)
}

pub fn render(frame:&mut Frame, display:&Display<WindowSurface>, sim_state:&mut SimState, circle_shader: &Program){
    let vertices = construct_vertices(&sim_state.objects);
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).expect("Failed to create vertex buffer");
    let indices = NoIndices(glium::index::PrimitiveType::Points);

    let dimensions = frame.get_dimensions();
    let uniforms = uniform! {
        scale: sim_state.scale,
        res: [dimensions.0 as f32,dimensions.1 as f32],
    };
    frame.draw(&vertex_buffer, &indices, circle_shader, &uniforms,
                &Default::default()).unwrap();
}

pub fn construct_vertices(objects:&Vec<Object>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for obj in objects {
        vertices.push(Vertex{
            position: [obj.pos_current.x, obj.pos_current.y],
        });
    }
    vertices
}

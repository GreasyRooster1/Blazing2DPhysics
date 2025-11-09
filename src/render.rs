use glium::{implement_vertex, Display, Frame, Surface};
use glium::glutin::surface::WindowSurface;
use crate::simulation::{Object, SimState};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub fn render(frame:&mut Frame, display: Display<WindowSurface>, sim_state: &mut SimState) {
    let vertices = construct_vertices(&sim_state.objects);
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices);
}

pub fn construct_vertices(objects:&Vec<Object>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for obj in objects {
        vertices.push(Vertex{
            position: [obj.pos_current.x, obj.pos_current.y, 0.0],
        });
    }
    vertices
}
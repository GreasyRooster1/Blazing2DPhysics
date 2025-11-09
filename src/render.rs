use glium::{Frame, Surface, Vertex};
use crate::simulation::{Object, SimState};

pub fn render(frame:&mut Frame, sim_state: &mut SimState) {
    let vertices = construct_vertices(&sim_state.objects).as_slice();
    
}

pub fn construct_vertices(objects:&Vec<Object>) -> Vec<f32> {
    let mut vertices = Vec::new();
    for obj in objects {
        vertices.push(obj.pos_current.x);
        vertices.push(obj.pos_current.y);
    }
    vertices
}
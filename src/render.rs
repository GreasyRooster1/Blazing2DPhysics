use glium::{Frame, Surface};
use crate::simulation::SimState;

pub fn render(frame:&mut Frame, sim_state: &mut SimState) {
    frame.clear_color(0.0,0.0, 0.0, 1.0);
}
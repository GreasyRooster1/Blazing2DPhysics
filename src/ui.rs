use imgui::{Condition, Ui};
use crate::simulation::SimState;

pub fn render_ui(ui: &mut Ui, sim_state: &mut SimState, elapsed: u64){
    timing_window(ui,sim_state,elapsed);
    stats_window(ui,sim_state,elapsed);
}


pub fn stats_window(ui: &mut Ui, sim_state: &mut SimState, elapsed: u64){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("obj count: {}",sim_state.objects.len()));
            ui.text(format!("scale: {}",sim_state.scale));
        });
}


pub fn timing_window(ui: &mut Ui, sim_state: &mut SimState, elapsed: u64){
    ui.window( "Timing")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Delta time: ~{}ns",elapsed));
            
            let fps = 1_000_000_000.0/elapsed as f32;
            ui.text(format!("Fps: ~{}",fps));

            ui.text(format!("Delta time: ~{}ns",elapsed));

            let fps = 1_000_000_000.0/elapsed as f32;
            ui.text(format!("Fps: ~{}",fps));
            ui.text(format!("step: {}",sim_state.step));
        });
}

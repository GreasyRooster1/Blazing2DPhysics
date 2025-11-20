use imgui::{Condition, Ui};
use crate::simulation::SimState;

pub fn render_ui(ui: &mut Ui, sim_state: &mut SimState, elapsed: u64){
    timing_window(ui,elapsed);
    stats_window(ui,sim_state,elapsed);
}


pub fn stats_window(ui: &mut Ui, sim_state: &mut SimState, elapsed: u64){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("∆ t: {}",sim_state.step));
            ui.text(format!("step: {}",sim_state.step));
            ui.text(format!("obj count: {}",sim_state.objects.len()));
            ui.text(format!("scale: {}",sim_state.scale));
        });
}


pub fn timing_window(ui: &mut Ui, elapsed: u64){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Delta time: ~{}ms",elapsed));
            ui.text(format!("Fps: ~{}",1/(1000*elapsed)));
        });
}

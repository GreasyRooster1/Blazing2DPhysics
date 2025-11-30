use imgui::{Condition, Ui};
use crate::simulation::SimState;

pub fn render_ui(ui: &mut Ui, sim_state: &mut SimState){
    timing_window(ui,sim_state);
    stats_window(ui,sim_state);
}


pub fn stats_window(ui: &mut Ui, sim_state: &mut SimState){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("obj count: {}",sim_state.objects.len()));
            ui.text(format!("scale: {}",sim_state.scale));
        });
}


pub fn timing_window(ui: &mut Ui, sim_state: &mut SimState){
    ui.window( "Timing")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Frame delta time: ~{}ns",sim_state.timings.frame_elapsed));
            
            let fps = 1_000_000_000.0/sim_state.timings.frame_elapsed as f32;
            ui.text(format!("Fps: ~{}",fps));

            ui.text(format!("Sim delta time: ~{}ns",sim_state.timings.sim_elapsed));

            let ups = 1_000_000_000.0/sim_state.timings.sim_elapsed as f32;
            ui.text(format!("updates/s: ~{}",ups));
            ui.text(format!("step: {}",sim_state.step));
        });
}

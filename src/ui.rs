use imgui::{Condition, Ui};
use crate::simulation::SimState;
use crate::{TARGET_FPS, TARGET_SIM_UPDATES};

pub fn render_ui(ui: &mut Ui, sim_state: &mut SimState){
    timing_window(ui,sim_state);
    stats_window(ui,sim_state);
}


pub fn stats_window(ui: &mut Ui, sim_state: &mut SimState){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .position([0.0, 230.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("obj count: {}",sim_state.objects.len()));
            ui.text(format!("scale: {}",sim_state.scale));
        });
}


pub fn timing_window(ui: &mut Ui, sim_state: &mut SimState){
    ui.window( "Timing")
        .size([300.0, 230.0], Condition::FirstUseEver)
        .position([0.0, 0.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Frame render time: ~{}ns",sim_state.timings.frame_calc_elapsed));
            ui.text(format!("Frame full time: ~{}ns",sim_state.timings.frame_full_elapsed));
            let fps = 1_000_000_000.0/sim_state.timings.frame_full_elapsed as f32;
            ui.text(format!("Fps: ~{}",fps));
            let fps_p = 1_000_000_000.0/sim_state.timings.frame_calc_elapsed as f32;
            ui.text(format!("max Fps: ~{}",fps_p));
            ui.text(format!("target fps: {}",TARGET_FPS));


            ui.text(format!("Sim calc time: ~{}ns",sim_state.timings.sim_calc_elapsed));
            ui.text(format!("Sim delta time: ~{}ns",sim_state.timings.sim_full_elapsed));
            let ups = 1_000_000_000.0/sim_state.timings.sim_full_elapsed as f32;
            ui.text(format!("updates/s: ~{}",ups));
            let ups_p = 1_000_000_000.0/sim_state.timings.sim_calc_elapsed as f32;
            ui.text(format!("max u/s: ~{}",ups_p));
            ui.text(format!("target updates: {}",TARGET_SIM_UPDATES));
            ui.text(format!("step: {}",sim_state.step));
        });
}

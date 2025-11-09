use imgui::{Condition, Ui};
use crate::simulation::SimState;

pub fn render_ui(ui: &mut Ui, sim_state: &mut SimState){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("step: {}",sim_state.step));
            ui.text(format!("obj count: {}",sim_state.objects.len()));
            ui.text(format!("scale: {}",sim_state.scale));
        });
}

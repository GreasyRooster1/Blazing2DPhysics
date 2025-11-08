use std::sync::Mutex;

pub struct SimState{

    pub(crate) step:u32,
}

impl SimState{
    pub fn start() -> Self{
        SimState{
            step:0
        }
    }
}


pub fn simulation_step(sim_state: &mut SimState){
    sim_state.step += 1;
}
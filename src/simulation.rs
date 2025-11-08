use std::sync::Mutex;
use glam::Vec2;

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

pub struct Object{
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub acceleration: Vec2,

    pub size:f32,
}


pub fn simulation_step(sim_state: &mut SimState){
    sim_state.step += 1;
}
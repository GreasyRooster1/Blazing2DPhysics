use std::sync::Mutex;
use glam::Vec2;

pub struct SimState{
    pub step:u32,
    pub objects:Vec<Object>,
}

impl SimState{
    pub fn start() -> Self{
        SimState{
            step:0,
            objects: vec![],
        }
    }
}

pub struct Object{
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub acceleration: Vec2,

    pub radius:f32,
}

impl Object {
    pub fn new(pos_current: Vec2) -> Self{
        Object{
            pos_current,
            pos_old: pos_current,
            acceleration: Vec2::ZERO,

            radius:1.0,
        }
    }
}

pub fn simulation_step(sim_state: &mut SimState){
    sim_state.step += 1;
}
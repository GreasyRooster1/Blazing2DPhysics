use std::cmp::max;
use std::sync::Mutex;
use glam::Vec2;
use rand::Rng;
use crate::GRAVITY;

#[derive(Clone)]
pub struct SimState{
    pub timings: Timings,
    pub step:u32,
    pub objects:Vec<Object>,
    pub scale: f32
}

impl SimState{
    pub fn start() -> Self{
        SimState{
            timings: Default::default(),
            step:0,
            objects: create_random_objects(100),
            scale: 100.0
        }
    }
}

#[derive(Clone, Default)]
pub struct Timings{
    pub frame_full_elapsed:u64,
    pub frame_calc_elapsed:u64,
    pub sim_full_elapsed:u64,
    pub sim_calc_elapsed:u64,
}

#[derive(Clone)]
pub struct Object{
    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub acceleration: Vec2,

    pub radius:f32,
}

impl Object {
    pub fn new(pos_current: Vec2) -> Self{
        Object{
            pos_current:pos_current.clone(),
            pos_old: pos_current.clone(),
            acceleration: Vec2::ZERO,

            radius:1.0,
        }
    }

    pub fn update(&mut self,dt_squared:f32){
        let velocity = self.get_vel();
        self.pos_old = self.pos_current;
        self.pos_current+= velocity+self.acceleration*(dt_squared);

        self.acceleration = Vec2::ZERO;
    }

    pub fn apply_forces(&mut self){
        self.accelerate(Vec2::new(0.0,GRAVITY));
    }

    pub fn apply_border_constraint(&mut self){
        let center = Vec2::ZERO;
        let radius: f32 = 100.0;

        let axis = center - self.pos_current;
        let dist = axis.length_squared().max(0.0001);
        if dist > radius.powf(2.0){
            let n = Vec2::new(axis.x/dist, axis.y/dist);
            let delta = radius+self.radius - dist;
            self.pos_current -= n*delta;
        }
    }

    pub fn get_vel(&self) -> Vec2 {
        self.pos_current-self.pos_old
    }

    pub fn accelerate(&mut self, acc:Vec2){
        self.acceleration += acc;
    }
}

pub fn create_random_objects(amt:i32) -> Vec<Object>{
    let mut rng = rand::rng();
    let mut objs = vec![];
    for _ in 0..amt{
        objs.push(Object::new(
            Vec2::new(
            rng.random_range(-100.0..100.0),
            rng.random_range(-100.0..100.0)
            )))
    }

    objs
}

pub fn simulation_step(sim_state: &mut SimState){
    let dt =  sim_state.timings.sim_full_elapsed as f32/1_000_000_000.0;
    let dt_squared =dt*dt;

    for obj in &mut sim_state.objects{
        obj.apply_forces();
        obj.apply_border_constraint();
        obj.update(dt_squared);
    }

    sim_state.step += 1;
}
use std::cmp::max;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use glam::{Vec2, Vec3};
use rand::Rng;
use crate::GRAVITY;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct SimState{
    pub timings: Timings,
    pub input: Input,
    pub step:u32,
    pub objects:Vec<Object>,
    pub scale: f32
}

impl SimState{
    pub fn start() -> Self{
        SimState{
            timings: Default::default(),
            input:Default::default(),
            step:0,
            objects: create_random_objects(200),
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

#[derive(Clone, Default)]
pub struct Input{
    pub mouse_pos:Vec2,
}

#[derive(Clone)]
pub struct Object{
    pub id:usize,

    pub pos_current: Vec2,
    pub pos_old: Vec2,
    pub acceleration: Vec2,

    pub radius:f32,
    pub color:Vec3,
}

impl Object {
    pub fn new(pos_current: Vec2) -> Self{
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Object{
            id,

            pos_current:pos_current.clone(),
            pos_old: pos_current.clone(),
            acceleration: Vec2::ZERO,

            radius:1.0,
            color:get_color(id),
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
        let dist = axis.length().max(0.0001);
        if dist > radius+self.radius{
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

pub fn get_color(id:usize) -> Vec3{
    let phase = 100.0;
    let t = (id as f32%phase)/phase;
    let r = f32::sin(t);
    let g = f32::sin(t + 0.33 * 2.0 * PI);
    let b = f32::sin(t + 0.66 * 2.0 * PI);

    Vec3::new(r,g,b)
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

    for obj in &mut sim_state.objects {
        obj.apply_forces();
        obj.apply_border_constraint();
    }
    solve_collisions_naive(sim_state);
    for obj in &mut sim_state.objects {
        obj.update(dt_squared);
    }

    sim_state.step += 1;
}

pub fn solve_collisions_naive(sim_state: &mut SimState){
    let num = sim_state.objects.len();
    for i in 0..num{
        for j in 0..num{
            solve_collision_between_objects(sim_state, i, j);
        }
    }
}

pub fn solve_collision_between_objects(sim_state: &mut SimState, i:usize, j:usize){
    if i==j  {return}
    let obj1 = &sim_state.objects[i];
    let obj2 = &sim_state.objects[j];

    let axis = obj1.pos_current - obj2.pos_current;
    let dist = axis.length().max(0.0001);
    if dist < obj1.radius+obj2.radius{
        let n = Vec2::new(axis.x/dist, axis.y/dist);
        let delta = obj1.radius+obj2.radius - dist;
        sim_state.objects[i].pos_current += (n*delta)/2.0;
        sim_state.objects[j].pos_current -= (n*delta)/2.0;
    }
}
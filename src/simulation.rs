use core::f32::math::ceil;
use std::cmp::max;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use glam::{Vec2, Vec3};
use glium::winit::dpi::Size;
use rand::{rng, Rng};
use crate::GRAVITY;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct SimState{
    pub step:u32,

    pub objects:Vec<Object>,
    pub collision_cells:Vec<CollisionCell>,
    pub collision_grid_width:i32,

    pub timings: Timings,
    pub input: Input,
    pub scale: f32,
    pub window_dimensions: Vec2
}

impl SimState{
    pub fn start() -> Self{
        SimState{
            timings: Default::default(),
            input:Default::default(),
            step:0,
            objects: create_random_objects(300),
            scale: 100.0,
            window_dimensions: Vec2::ZERO,
            collision_cells: CollisionCell::create_cells(2),
            collision_grid_width: 200/2
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

#[derive(Clone,Copy)]
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

            radius:rng().random_range(1.0..2.0),
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

        let axis = self.pos_current-center;
        let dist = axis.length().max(0.0001);
        if dist > radius-self.radius{
            let n = Vec2::new(axis.x/dist, axis.y/dist);
            let delta = radius-self.radius;
            self.pos_current = center+n*delta;
        }
    }
    pub fn apply_mouse_constraint(&mut self,mouse_pos: Vec2){

        let axis = self.pos_current-mouse_pos;
        let dist = axis.length().max(0.0001);
        if dist < 5.0+self.radius{
            let n = Vec2::new(axis.x/dist, axis.y/dist);
            let delta = 5.0+self.radius;
            self.pos_current = mouse_pos+n*delta;
        }
    }

    pub fn get_vel(&self) -> Vec2 {
        self.pos_current-self.pos_old
    }

    pub fn accelerate(&mut self, acc:Vec2){
        self.acceleration += acc;
    }
}

#[derive(Clone)]
struct CollisionCell{
    pub objects:Vec<usize>,
    pub position:Vec2,
    pub size:Vec2,
}

impl CollisionCell{
    pub fn new(position:Vec2, size: Vec2) -> Self{
        CollisionCell {
            objects:Vec::new(),
            position,
            size,
        }
    }

    pub fn create_cells(size:i32) -> Vec<CollisionCell>{
        let grid_size = 200/size;
        let mut cells = Vec::with_capacity((grid_size * grid_size) as usize);
        for i in 0..grid_size{
            for j in 0..grid_size{
                cells.push(CollisionCell::new(
                    Vec2::new((i * size) as f32, (j * size * grid_size) as f32),
                    Vec2::new(grid_size as f32, grid_size as f32),
                ))
            }
        }

        cells
    }

    pub fn populate_objects(&mut self,sim_state: &SimState){
        for i in 0..sim_state.objects.len(){
            let obj = sim_state.objects[i];
            if obj.pos_current.x > self.position.x && obj.pos_current.y > self.position.y && obj.pos_current.x < self.position.x+self.size.x && obj.pos_current.y < self.position.y+self.size.y{
                self.objects.push(i);
            }
        }
    }

    pub fn solve_collision_with_cell(&mut self,sim_state: &mut SimState, other_index: i32){
        let other = &sim_state.collision_cells[other_index as usize];
        let other_len = other.objects.len();
        for i in 0..self.objects.len(){
            for j in 0..other_len{
                solve_collision_between_objects(&mut sim_state.objects, i, j);
            }
        }
    }
}

pub fn get_color(id:usize) -> Vec3{
    let phase = 100.0;
    let t = (id as f32%phase)/phase * 2.0*PI;
    let r = f32::sin(t);
    let g = f32::sin(t + 0.33 * 2.0 * PI);
    let b = f32::sin(t + 0.66 * 2.0 * PI);

    Vec3::new(r*r,g*g,b*b)
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
        obj.apply_mouse_constraint(sim_state.input.mouse_pos)
    }
    solve_collisions_naive(sim_state);
    for obj in &mut sim_state.objects {
        obj.update(dt_squared);
    }

    sim_state.step += 1;
}

pub fn solve_collisions_cells(sim_state: &mut SimState){
    for i in 0..sim_state.collision_grid_width{
        for j in 0..sim_state.collision_grid_width{
            let current_cell_index = i * sim_state.collision_grid_width + j;
            let current_cell = &mut sim_state.collision_cells[current_cell_index as usize];
            for k in -1..=1{
                for l in -1..=1{
                    let other_cell_index = k * sim_state.collision_grid_width + l;

                    current_cell.solve_collision_with_cell(sim_state, other_cell_index);
                }
            }
        }
    }
}

pub fn solve_collisions_naive(sim_state: &mut SimState){
    let num = sim_state.objects.len();
    for i in 0..num{
        for j in 0..num{
            solve_collision_between_objects(&mut sim_state.objects, i, j);
        }
    }
}

pub fn solve_collision_between_objects(sim_state_objects: &mut Vec<Object>, i:usize, j:usize){
    if i==j  {return}
    let obj1 = &sim_state_objects[i];
    let obj2 = &sim_state_objects[j];

    let axis = obj1.pos_current - obj2.pos_current;
    let dist = axis.length().max(0.0001);
    if dist < obj1.radius+obj2.radius{
        let n = Vec2::new(axis.x/dist, axis.y/dist);
        let delta = obj1.radius+obj2.radius - dist;
        sim_state_objects[i].pos_current += (n*delta)/2.0;
        sim_state_objects[j].pos_current -= (n*delta)/2.0;
    }
}
#![feature(random)]

mod support;
mod simulation;
mod render;

use std::sync::{Arc, Mutex};
use std::thread;
use imgui::*;
use lazy_static::lazy_static;
use crate::render::render;
use crate::simulation::{simulation_step, SimState};

lazy_static! {
    static ref sim_state_mutex: Mutex<SimState> = Mutex::new(SimState::start());
}

fn main() {


    thread::spawn(move || {
        loop {
            simulation_step(&mut *sim_state_mutex.lock().unwrap());
        }
    });

    support::simple_init(file!(), move |_, ui,state| {
        render_ui(ui,state)
    }, |frame,display,state|{
        render(frame,display,state);
    });
}

fn render_ui(ui: &mut Ui,sim_state: &mut SimState){
    ui.window( "Stats")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("step: {}",sim_state.step));
        });
}


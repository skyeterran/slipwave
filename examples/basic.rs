use slipwave::time::{State, Loop};
use slipwave::log::{Logger};
use std::time::{Duration, Instant};

fn main() {
    println!("Slipwave Engine | 2021 | Skye Terran");

    // Create a logger
    let log_time = Logger::new("time");
    
    // create a sim loop
    log_time.print("Creating sim loop...");
    let mut sim = Loop::new();

    // set if the sim is realtime or as fast as possible
    sim.set_realtime(true);

    // set the loop update interval
    sim.set_update_interval(Duration::from_millis(40));

    // set the loop's timescale
    sim.get_state_mut().set_timescale(1.0);

    // datastream
    let mut velocity: f32 = 100.0;
    
    // execute the sim loop
    log_time.print("Executing loop...");
    loop {
        
        // step the sim forward
        sim.step();
        
        // update logic goes here
        if sim.is_awake() {
           velocity -= 9.8 * sim.get_state().get_timestep();
           //println!("{}", velocity);
        }
        sim.get_state().debug_time();

        if velocity <= 50.0 {
            sim.get_state().debug_time();
            break;
        }
    }
}
use std::env;
use slipwave::time::{State, Loop};
use slipwave::log::{Logger};
use slipwave::vcr::{ComputeObject};

fn main() {
    println!("Slipwave Engine | 2021 | Skye Terran");

    // Create a logger
    let log_time = Logger::new("time");
    
    // create a sim loop
    log_time.print("Creating sim loop...");
    let mut sim = Loop::new();

    // set the loop update interval
    sim.set_update_interval(40);

    // set the loop's timescale
    sim.get_state_mut().set_timescale(1.0);

    // datastream
    let mut x: i32 = 0;

    // Create a compute object
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    
    // execute the sim loop
    log_time.print("Executing loop...");
    loop {
        
        // step the sim forward
        sim.step();
        
        // update logic goes here
        if sim.is_awake() {
            // Create and execute a compute object
            let mut vm = ComputeObject::from_file(file_path);
            println!("{:?}", vm.execute());
        }
        //sim.get_state().debug_time();

        // display logic goes here
        // problem: the timestep is not what we want here. we need to get the elapsed time 
        //let timestep = sim.get_state().get_timestep();
        //let x_interpolated: f32 = x as f32 + timestep;
        //println!("x: {}", x_interpolated);
    }
}
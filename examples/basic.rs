use slipwave::time::{State, Loop};

fn main() {
    println!("Slipwave Engine | 2021 | Skye Terran");

    // create a sim loop
    let mut sim = Loop::new();

    // set the loop update interval
    sim.set_update_interval(40);

    // set the loop's timescale
    sim.get_state_mut().set_timescale(1.0);

    // datastream
    let mut x: i32 = 0;

    // execute the sim loop
    loop {
        // step the sim forward
        sim.step();
        
        // update logic goes here
        if sim.is_awake() {
            
            //x += 1;
            //println!("x: {}", x);
        }
        sim.get_state().debug_time();

        // display logic goes here
        // problem: the timestep is not what we want here. we need to get the elapsed time 
        //let timestep = sim.get_state().get_timestep();
        //let x_interpolated: f32 = x as f32 + timestep;
        //println!("x: {}", x_interpolated);
    }
}
use slipwave::core::{State, Loop};

fn main() {
    println!("Slipwave Engine | 2021 | Skye Terran");

    // create a sim loop
    let mut sim = Loop::new();

    // datastream
    let mut x: i32 = 0;

    // execute the sim loop
    loop {
        // step the sim forward
        sim.step();
        
        // update logic goes here
        if sim.is_awake() {
            //sim.get_state().debug_time();

            x += 1;
            println!("x: {}", x);
        }

        // display logic goes here
    }
}
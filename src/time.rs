use std::time::{Duration, Instant};

/// Contains mutable simulation state which can be changed via callback functions
#[derive(Copy, Clone)]
pub struct State {
    timescale: f32,
    simulate: bool,
    clock_start: Instant,
    last_tick: Instant,
    delta_time: u32,
    timestep: f32,
    irl_time: Duration,
    sim_time: Duration
}

impl State {
    /// Creates a default State object
    pub fn new() -> State {
        // Create default state object
        let new_state = State {
            timescale: 1.0,
            simulate: true,
            clock_start: Instant::now(),
            last_tick: Instant::now(),
            delta_time: 0,
            timestep: 0.0,
            irl_time: Duration::new(0,0),
            sim_time: Duration::new(0,0)
        };

        // Return this default state
        new_state
    }

    /// Returns the current "delta time", the real time (in ms) elapsed since the last update tick
    pub fn get_delta_time(self) -> u32 {
        self.delta_time
    }

    /// Returns the current "timestep", the virtual time (in s) elapsed since the last update tick (necessary for scaling physics simulations, etc.)
    pub fn get_timestep(self) -> f32 {
        self.timestep
    }

    /// Returns the current real time elapsed since the start of the simulation
    pub fn get_irl_time(self) -> Duration {
        self.irl_time
    }

    /// Returns the current simulation time elapsed since the start of the simulation
    pub fn get_sim_time(self) -> Duration {
        self.sim_time
    }

    /// Returns the current "timescale", the speed of simulation time relative to real time
    pub fn get_timescale(self) -> f32 {
        self.timescale
    }

    /// Returns the time of the last tick
    pub fn get_last_tick(self) -> Instant {
        self.last_tick
    }

    /// Pauses the simulation from within update logic
    pub fn pause(&mut self) {
        self.simulate = false;
    }

    /// Resumes the simulation from within update logic
    pub fn resume(&mut self) {
        self.simulate = true;
    }

    /// Changes the simulation timescale
    pub fn set_timescale(&mut self, timescale: f32) {
        self.timescale = timescale;
    }

    /// Prints a string of information about the current step's timings
    ///
    /// # Example:
    /// `IRL time: 4443ms | Sim time: 4443ms | Delta time (tick): 40ms | Delta time (step): 40.0638ms | Timestep: 0.04s`
    /// # Terminology:
    /// - *IRL time:* Real time (in ms) elapsed since the start of the simulation
    /// - *Sim time:* Virtual time (in ms) elapsed since the start of the simulation
    /// - *Delta time (tick):* Real time (in ms) elapsed between the last tick and the previous tick
    /// - *Timestep:* Virtual time (in s with ms accuracy) elapsed since the last tick
    pub fn debug_time(self) {
        let elapsed_time = Instant::now().duration_since(self.last_tick);
        println!("IRL time: {}ms | Sim time: {}ms | Delta time: {}ms | Timestep: {}", self.irl_time.as_millis(), self.sim_time.as_millis(), self.delta_time, self.timestep);
    }
}

/// The simulation loop itself
pub struct Loop {
    state: State,
    realtime: bool,
    update_interval: u32,
    awake: bool
}

impl Loop {
    /// Creates a new simulation with default values
    pub fn new() -> Loop {
        // Create a new State object
        let mut new_state = State::new();
        
        // Create a Loop object with a default State
        let mut new_loop = Loop {
            state: new_state,
            realtime: true,
            update_interval: 40,
            awake: false
        };
        
        // Initialize the delta time to be the same as the update interval (to prevent division by zero)
        new_loop.state.delta_time = new_loop.update_interval;

        // Initialize the timestep based on the new delta time
        new_loop.state.timestep = 0.0;

        // Return the now-initialized Loop
        new_loop
    }

    /// Initializes or re-initializes the simulation
    pub fn init(&mut self) {
        // Make sure the simulation will run
        self.state.simulate = true;

        // reset the internal clocks
        self.state.clock_start = Instant::now();
        self.state.irl_time = Duration::new(0,0);
        self.state.sim_time = Duration::new(0,0);
    }

    /// Returns whether the loop is currently "awake" (logic should occur)
    pub fn is_awake(&self) -> bool {
        self.awake
    }

    /// Returns an immutable reference to the Loop's current State object
    pub fn get_state(&self) -> &State {
        &self.state
    }

    /// Returns a mutable reference to the Loop's State object
    pub fn get_state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Executes the per-loop logic (can be triggered manually so that hypoloop can be tied into external event loops)
    // TODO - support frameskips
    pub fn step(&mut self) {
        // don't run if the simulation is paused
        if self.state.simulate {
            // track elapsed real time each step
            let elapsed_time = Instant::now().duration_since(self.state.last_tick);

            if !self.realtime || delta_time(self.state.last_tick) >= self.update_interval {    
                // update clocks
                if self.realtime {
                    self.state.delta_time = delta_time(self.state.last_tick);
                    self.state.sim_time += elapsed_time.mul_f32(self.state.timescale);
                    self.state.irl_time += elapsed_time;
                } else {
                    self.state.delta_time = self.update_interval;
                    self.state.sim_time += Duration::from_millis(self.update_interval as u64);
                    self.state.irl_time = Instant::now().duration_since(self.state.clock_start);
                }
                
                // mark the loop as "awake", meaning update logic should occur
                self.awake = true;
                
                // record last tick time
                self.state.last_tick = Instant::now();
            } else {
                // mark the loop as "asleep", meaning update logic should NOT occur
                self.awake = false;
            }
            // compute the current timestep (a float describing the virtual time since last tick, in ticks)
            let mut current_timestep = (elapsed_time.as_millis() as f32) / (self.update_interval as f32);
            // prevent a timestep of 1.0 (which will throw off interpolation)
            if current_timestep >= 1.0 {
                current_timestep = 0.0;
            }
            // update the sim timestep
            self.state.timestep = current_timestep;
        }
    }

    /// Turns real-time mode on/off
    pub fn set_realtime(&mut self, realtime: bool) {
        self.realtime = realtime;
    }

    /// Returns the "update interval", the minimum time (in ms) which will elapse between update ticks
    pub fn get_update_interval(self) -> u32 {
        self.update_interval
    }

    /// Changes the update interval
    pub fn set_update_interval(&mut self, update_interval: u32) {
        self.update_interval = update_interval;
    }
}

// gets the real time (in ms) that's elapsed since the earlier Instant
fn delta_time(earlier: Instant) -> u32 {
    Instant::now().duration_since(earlier).as_millis() as u32
}
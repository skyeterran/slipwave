use hypoloop::core::{State, Loop};
use slipwave::state::{World, Health, Name};

fn main() {
    // create a new sim loop
    let mut sim = Loop::new();
    sim.set_update_interval(20);

    // create the game world
    let mut world = World::new();

    // add entities to the game world
    // Icarus's health is *not* looking good.
    world.new_entity(Some(Health(-10)), Some(Name("Icarus"))); 

    // Prometheus is very healthy.
    world.new_entity(Some(Health(100)), Some(Name("Prometheus"))); 

    // Note that Zeus does not have a `Health` component.
    world.new_entity(None, Some(Name("Zeus"))); 

    // create a closure containing your update logic
    let mut tick = move |state: &mut State| {    
        let zip = world
            .health_components
            .iter()
            .zip(world.name_components.iter());

        let with_health_and_name =
            zip.filter_map(|(health, name): (&Option<Health>, &Option<Name>)| {
                Some((health.as_ref()?, name.as_ref()?))
            });

        // health system
        for (health, name) in with_health_and_name {
            if health.0 < 0 {
                println!("{} has perished!", name.0);
            } else {
                println!("{} is still alive!", name.0);
            }
        }


        // print information about the current tick's timings
        state.debug_time();
    };
    
    // create a closure containing your display logic
    let mut display = move |state: &mut State| {
        //
    };

    // run the simulation with your user-defined update and display logic
    // initialize the sim (cleans internal clocks, etc.)
    sim.init();
    loop {
        // "step" the sim forward
        sim.step(&mut tick, &mut display);
    }
}
pub mod state {
    pub struct Health(pub i32);
    pub struct Name(pub &'static str);

    pub struct World {
        pub health_components: Vec<Option<Health>>,
        pub name_components: Vec<Option<Name>>,
    }
    impl World {
        pub fn new() -> Self {
            Self {
                health_components: Vec::new(),
                name_components: Vec::new(),
            }
        }

        pub fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
            self.health_components.push(health);
            self.name_components.push(name);
        }
    }
}
pub struct Logger {
    name: String
}

impl Logger {
    pub fn new(name: &str) -> Logger {
        Logger { name: name.to_string() }
    }

    pub fn print(&self, output: &str) {
        println!("[{}] {}", self.name, output);
    }
}
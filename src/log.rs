pub struct Logger {
    categories: Vec<String>
}

impl Logger {
    pub fn print(self, output: &String, category: &String) {
        if self.categories.contains(category) {
            println!(output);
        }
    }
}
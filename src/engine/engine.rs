/// Engine module.
/// This module provides the core features of the game engine.
/// - Event loop
/// - Game loop
/// - Window management
#[derive(Debug)]
pub struct Engine {
    /// The name of the application.
    name: String,
}

impl Engine {
    /// Create a new instance of the engine.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn start(&self) {
        println!("Starting the engine: {}", self.name);
    }
}

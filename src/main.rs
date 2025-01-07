mod engine;

const APPLICATION_NAME: &str = "Original Game Engine";

fn main() {
    let engine = engine::engine::Engine::new(APPLICATION_NAME);
    match engine.start() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

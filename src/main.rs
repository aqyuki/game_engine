mod engine;

const APPLICATION_NAME: &str = "Original Game Engine";

fn main() {
    let engine = engine::engine::Engine::new(APPLICATION_NAME);
    engine.start();
}

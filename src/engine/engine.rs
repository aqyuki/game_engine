use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
};

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

    pub fn start(&self) -> anyhow::Result<()> {
        println!("Starting the engine: {}", self.name);

        // SDL2の初期化
        let sdl_context = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(e) => return Err(anyhow::anyhow!("Failed to initialize SDL2: {}", e)),
        };
        let video_subsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to initialize video subsystem: {}",
                    e
                ))
            }
        };

        // windowの初期化
        let window = match video_subsystem
            .window(&self.name, 800, 600)
            .vulkan()
            .resizable()
            .position_centered()
            .fullscreen_desktop()
            .build()
        {
            Ok(window) => window,
            Err(e) => return Err(anyhow::anyhow!("Failed to create window: {}", e)),
        };

        // canvasの作成
        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(anyhow::anyhow!("Failed to create canvas: {}", e)),
        };

        // Event loop
        let mut event_pump = match sdl_context.event_pump() {
            Ok(event_pump) => event_pump,
            Err(e) => return Err(anyhow::anyhow!("Failed to get event pump: {}", e)),
        };

        // canvasは白背景とする
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    // Closeボタンが押される or ESCキーでアプリケーションを終了する
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    // F11キーでフルスクリーンモードを切り替える
                    Event::KeyDown {
                        keycode: Some(Keycode::F11),
                        ..
                    } => self.toggle_window_fullscreen(&mut canvas),

                    // windowのリサイズ時にcanvasをクリアする
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(..) => {
                            canvas.clear();
                            canvas.present();
                        }
                        _ => (),
                    },

                    _ => (),
                }
            }
        }

        Ok(())
    }

    fn toggle_window_fullscreen(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let window = canvas.window_mut();
        match window.fullscreen_state() {
            sdl2::video::FullscreenType::Off => {
                match window.set_fullscreen(sdl2::video::FullscreenType::Desktop) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Failed to set fullscreen: {}", e),
                }
            }
            _ => match window.set_fullscreen(sdl2::video::FullscreenType::Off) {
                Ok(_) => (),
                Err(e) => eprintln!("Failed to set windowed mode: {}", e),
            },
        }
    }
}

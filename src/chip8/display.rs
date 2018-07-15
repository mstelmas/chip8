use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// TODO: make it more generic
pub struct Display {
    buffer: [[u8; 64]; 32],
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let v_ctx = sdl_context.video().unwrap();

        // TODO: do not handle this here!
        let window = match v_ctx
            .window("chip8 VM", 640, 480)
            .position_centered()
            .build() {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err)
        };

        // TODO: do not handle this here!
        let mut canvas = match window
            .into_canvas()
            .present_vsync()
            .build() {
            Ok(canvas) => canvas,
            Err(err) => panic!("failed to create canvas: {}", err)
        };

        let _ = canvas.clear();
        let _ = canvas.present();

        Display {
            buffer: [[0; 64]; 32],
            canvas
        }
    }

    pub fn clear(&mut self) {

    }

    pub fn draw(&self) {

    }
}

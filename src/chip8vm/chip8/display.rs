use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// TODO: extract to config
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const SCALE: usize = 12;
const WINDOW_WIDTH: usize = CHIP8_WIDTH * SCALE;
const WINDOW_HEIGHT: usize = CHIP8_HEIGHT * SCALE;

// TODO: make it more generic
pub struct Display {
    // TODO: move vram to Interconnect?
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Display {
        let v_ctx = sdl_context.video().unwrap();

        // TODO: do not handle this here!
        let window = match v_ctx
            .window("chip8 VM", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
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
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            canvas
        }
    }

    pub fn vram(&mut self) -> &mut [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT] {
        &mut self.vram
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn draw(&mut self) {
        // TODO: enable custom color configuration
        let black: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0, 0, 0);
        let white: sdl2::pixels::Color = sdl2::pixels::Color::RGB(255, 255, 255);

        let mut pixel = Rect::new(0, 0, 0, 0);

        for i in 0..CHIP8_WIDTH {
            for j in 0..CHIP8_HEIGHT {
                pixel.set_x((i * SCALE) as i32);
                pixel.set_y((j * SCALE) as i32);
                pixel.set_width(SCALE as u32);
                pixel.set_height(SCALE as u32);

                if self.vram[j as usize][i as usize] == 1 {
                    self.canvas.set_draw_color(white);
                    self.canvas.fill_rect(pixel);
                } else {
                    self.canvas.set_draw_color(black);
                    self.canvas.fill_rect(pixel);
                };
            };
        };

        self.canvas.present()
    }
}

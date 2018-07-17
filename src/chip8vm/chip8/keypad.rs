use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum Keys {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

type KeypadState = [bool; 16];

pub struct Keypad {
    keypad: KeypadState,
    key_events: sdl2::EventPump
}

impl Keypad {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        Keypad {
            keypad: [false; 16],
            key_events: sdl_context.event_pump().unwrap()
        }
    }

    pub fn update_state(&mut self, keypad_state: KeypadState) {
        self.keypad = keypad_state;
    }

    // TODO: better abstraction
    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keypad[key]
    }

    pub fn poll(&mut self) -> Result<KeypadState, ()> {

        for event in self.key_events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Err(()),
                _ => {}
            };
        }

        let keys: Vec<Keycode> = self.key_events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut new_key_states = [false; 16];

        for key in keys {
            // TODO: allow to define custom key mappings
            let index = match key {
                Keycode::Num1 => Some(Keys::K1),
                Keycode::Num2 => Some(Keys::K2),
                Keycode::Num3 => Some(Keys::K3),
                Keycode::Num4 => Some(Keys::KC),
                Keycode::Q => Some(Keys::K4),
                Keycode::W => Some(Keys::K5),
                Keycode::E => Some(Keys::K6),
                Keycode::R => Some(Keys::KD),
                Keycode::A => Some(Keys::K7),
                Keycode::S => Some(Keys::K8),
                Keycode::D => Some(Keys::K9),
                Keycode::F => Some(Keys::KE),
                Keycode::Z => Some(Keys::KA),
                Keycode::X => Some(Keys::K0),
                Keycode::C => Some(Keys::KB),
                Keycode::V => Some(Keys::KF),
                _ => None,
            };

            if let Some(i) = index {
                new_key_states[i as usize] = true;
            }
        }

        Ok(new_key_states)
    }
}

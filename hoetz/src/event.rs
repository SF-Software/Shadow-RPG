use types::{MouseState, KeyCode};
use std::collections::HashMap;
use sdl2::EventPump;
use sdl2::event::Event;


pub trait KeyboardState {
    fn press(&self, key: KeyCode) -> bool;
    fn trigger(&self, key: KeyCode) -> bool;
    fn repeart(&self, key: KeyCode) -> bool;
}
pub struct UIInput<'a> {
    pub mouse: MouseState,
    pub keyboard: &'a KeyboardState,
}

impl KeyboardState for HashMap<KeyCode, i32> {
    fn press(&self, key: KeyCode) -> bool {
        self.contains_key(&key)
    }
    fn trigger(&self, key: KeyCode) -> bool {
        if let Some(v) = self.get(&key) {
            *v == 0
        } else {
            false
        }
    }
    fn repeart(&self, key: KeyCode) -> bool {
        if let Some(v) = self.get(&key) {
            *v == 0 || *v >= 10
        } else {
            false
        }
    }
}
pub struct EventSystem {
    keyboard_state: HashMap<KeyCode, i32>,
    event_pump: EventPump,
}

impl EventSystem {
    pub fn new(event_pump: EventPump) -> EventSystem {
        Self {
            keyboard_state: HashMap::new(),
            event_pump: event_pump,
        }
    }
    pub fn process(&mut self, running: &mut bool) -> UIInput {
        for keys in self.keyboard_state.values_mut() {
            *keys += 1;
        }
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    *running = false;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.keyboard_state.remove(&keycode);
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat,
                    ..
                } => {
                    if !(self.keyboard_state.contains_key(&keycode) || repeat) {
                        self.keyboard_state.insert(keycode, 0);
                    }
                }
                _ => {}
            }
        }
        UIInput {
            mouse: MouseState::new(&self.event_pump),
            keyboard: &self.keyboard_state,
        }
    }
}

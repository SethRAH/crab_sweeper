use ggez::input::mouse::{MouseButton};

use crate::ui_common::button_handler::{ButtonHandler, ButtonState};

pub struct MouseInputHandler {
    pub left_button_handler: ButtonHandler,
    pub right_button_handler: ButtonHandler,
    pub middle_button_handler: ButtonHandler,
    pub x: f32,
    pub y: f32
}

impl MouseInputHandler {
    pub fn new() -> Self {
        let left_button_handler = ButtonHandler::new();
        let right_button_handler = ButtonHandler::new();
        let middle_button_handler = ButtonHandler::new();

        MouseInputHandler {left_button_handler, right_button_handler, middle_button_handler, x: 0.0, y: 0.0}
    }

    pub fn record_button_click(&mut self, button: MouseButton, x: f32, y: f32, is_down: bool ) {
        let button_state = match is_down { 
            true => ButtonState::Pressed,
            false => ButtonState::Released
        };
        
        match button {
            MouseButton::Left => self.left_button_handler.push(button_state, x, y),
            MouseButton::Right => self.right_button_handler.push(button_state, x, y),
            MouseButton::Middle => self.middle_button_handler.push(button_state, x, y),
            _ => ()
        };

        self.x = x;
        self.y = y;
    }
    
    pub fn record_mouse_motion(&mut self, x: f32, y: f32){
        let left_button_state = match self.left_button_handler.past_state.is_some() {
            true => self.left_button_handler.past_state.unwrap(),
            false => ButtonState::Released
        };
        self.left_button_handler.push(left_button_state, x, y);

        let right_button_state = match self.right_button_handler.past_state.is_some() {
            true => self.right_button_handler.past_state.unwrap(),
            false => ButtonState::Released
        };
        self.right_button_handler.push(right_button_state, x, y);

        let middle_button_state = match self.middle_button_handler.past_state.is_some() {
            true => self.middle_button_handler.past_state.unwrap(),
            false => ButtonState::Released
        };
        self.middle_button_handler.push(middle_button_state, x, y);

        self.x = x;
        self.y = y;
    }

    pub fn clear_stored_positions(&mut self) {
        self.left_button_handler.clear_stored_positions();
        self.right_button_handler.clear_stored_positions();
        self.middle_button_handler.clear_stored_positions();
    }
}
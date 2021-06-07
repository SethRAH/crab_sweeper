use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Released = 0,
    Pressed = 1
}

pub struct ButtonHandler{
    pub past_state: Option<ButtonState>,
    pub is_initial_click: bool,
    pub is_held: bool,
    pub is_initial_release: bool,
    pub last_click_position: Option<Vec2>,
    pub last_release_position: Option<Vec2>
}

impl ButtonHandler {
    pub fn new() -> Self {
        ButtonHandler {
            past_state: None,
            is_initial_click: false,
            is_held: false,
            is_initial_release: false,
            last_click_position: None,
            last_release_position: None
        }
    }

    pub fn push(&mut self, new_state: ButtonState, x_pos: f32, y_pos: f32) {
        if self.past_state.is_some() {
            let old_state = self.past_state.unwrap();
            match new_state {
                ButtonState::Pressed => {
                    self.is_initial_click = match old_state {
                        ButtonState::Released => true,
                        ButtonState::Pressed => false
                    };
                    self.is_held = match old_state {
                        ButtonState::Released => false,
                        ButtonState::Pressed => true
                    };
                    self.is_initial_release = false;

                    if old_state == ButtonState::Released {
                        self.last_click_position = Some(glam::Vec2::new(x_pos, y_pos));
                    }
                },
                ButtonState::Released => {
                    self.is_initial_click = false;
                    self.is_held = false;
                    self.is_initial_release = match self.past_state.unwrap() {
                        ButtonState::Released => false,
                        ButtonState::Pressed => true
                    };

                    if self.past_state.unwrap() == ButtonState::Pressed {
                        self.last_release_position = Some(glam::Vec2::new(x_pos, y_pos));
                    }
                }
            }
        } else {
            match new_state {
                ButtonState::Pressed => {
                    self.is_initial_click = true;
                    self.is_held = false;
                    self.is_initial_release = false;
                    self.last_click_position = Some(glam::Vec2::new(x_pos, y_pos));
                },
                ButtonState::Released => {                    
                    self.is_initial_click = false;
                    self.is_held = false;
                    self.is_initial_release = true;
                    self.last_release_position = Some(glam::Vec2::new(x_pos, y_pos));
                }
            }
        }

        self.past_state = Some(new_state);
    }

    pub fn clear_stored_positions(&mut self) {
        self.last_release_position = None;
        self.last_click_position = None;
    }
}
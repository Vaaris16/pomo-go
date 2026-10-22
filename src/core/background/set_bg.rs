use egui::Color32;

use crate::{FOCUS_APP_BACKGROUND, PomoState, REST_APP_BACKGROUND};

pub fn set_bg(pomo_state: &PomoState) -> Color32 {
    match pomo_state {
        PomoState::Focus => FOCUS_APP_BACKGROUND,
        PomoState::Rest => REST_APP_BACKGROUND,
    }
}

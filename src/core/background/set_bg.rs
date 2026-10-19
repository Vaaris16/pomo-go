use egui::Color32;

use crate::PomoState;

pub fn set_bg(pomo_state: &PomoState) -> Color32 {
    match pomo_state {
        PomoState::Focus => Color32::RED,
        PomoState::Rest => Color32::BLACK,
    }
}

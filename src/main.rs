use eframe::{NativeOptions, egui};
use egui::Color32;

use crate::{core::pomo_timer::timer::Timer, ui::app_ui::pomo_ui};

mod core;
mod ui;

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<PomoGo>::default())),
    )
}

#[derive(Default)]
pub enum PomoState {
    #[default]
    Focus,
    Rest,
}

pub struct PomoGo {
    pomo_state: PomoState,
    timer: Timer,
}

impl Default for PomoGo {
    fn default() -> Self {
        Self {
            pomo_state: PomoState::Rest,
            timer: Timer::new(),
        }
    }
}

impl eframe::App for PomoGo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.timer.timer();
        pomo_ui(self, ui);
        ui.ctx().request_repaint();
    }
}

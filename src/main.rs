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

pub enum PomoState {
    Focus,
    Rest,
}

pub struct PomoGo {
    pomo_state: PomoState,
    timer: Timer,
    is_running: bool,
}

impl Default for PomoGo {
    fn default() -> Self {
        Self {
            pomo_state: PomoState::Focus,
            timer: Timer::new(),
            is_running: true,
        }
    }
}

impl eframe::App for PomoGo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.timer.timer(self.is_running);
        pomo_ui(self, ui);
        ui.ctx().request_repaint();
    }
}

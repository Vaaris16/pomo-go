use eframe::{NativeOptions, egui};
use egui::Color32;

use crate::{
    core::{fonts::setup_fonts::setup_fonts, pomo_timer::timer::Timer},
    ui::app_ui::pomo_ui,
};

mod core;
mod ui;

pub const FOCUS_APP_BACKGROUND: Color32 = Color32::from_rgb(187, 84, 75);
pub const REST_APP_BACKGROUND: Color32 = Color32::from_rgb(90, 129, 102);

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };
    eframe::run_native(
        "App",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::<PomoGo>::default())
        }),
    )
}

pub enum PomoState {
    Focus,
    Rest,
}

impl PomoGo {
    fn toggle_timer_state(&mut self) {
        self.is_running = match self.is_running {
            PomoTimerState::Paused => PomoTimerState::Running,
            PomoTimerState::Running => PomoTimerState::Paused,
            PomoTimerState::Finished => PomoTimerState::Finished,
        }
    }
}

pub enum PomoTimerState {
    Running,
    Paused,
    Finished,
}

pub struct PomoGo {
    pomo_state: PomoState,
    timer: Timer,
    is_running: PomoTimerState,
}

impl Default for PomoGo {
    fn default() -> Self {
        Self {
            pomo_state: PomoState::Focus,
            timer: Timer::new(),
            is_running: PomoTimerState::Paused,
        }
    }
}

impl eframe::App for PomoGo {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.timer.timer(&self.is_running);
        pomo_ui(self, ui);
        ui.ctx().request_repaint();
    }
}

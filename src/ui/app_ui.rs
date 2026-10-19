use std::time::Instant;

use egui::{Color32, Ui};

use crate::{
    PomoGo, PomoState,
    core::{background::set_bg::set_bg, pomo_timer::timer::Timer},
};

pub fn pomo_ui(pomo_go: &mut PomoGo, ui: &mut Ui) {
    let bg = set_bg(&pomo_go.pomo_state);

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(bg))
        .show(ui, |ui| {
            pomo_timer(&mut pomo_go.timer, ui);
        });
}

fn pomo_timer(timer: &mut Timer, ui: &mut Ui) {
    let (m, s) = timer.timer();

    let time = format!("{:2}:{:2}", m, s);

    ui.centered_and_justified(|ui| {
        ui.label(egui::RichText::new(time).size(200.).color(Color32::WHITE))
    });
}

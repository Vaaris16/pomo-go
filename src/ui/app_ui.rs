use egui::{Color32, Ui};

use crate::{
    PomoGo,
    core::{background::set_bg::set_bg, pomo_timer::timer::Timer},
};

pub fn pomo_ui(pomo_go: &mut PomoGo, ui: &mut Ui) {
    let bg = set_bg(&pomo_go.pomo_state);

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(bg))
        .show(ui, |ui| {
            pomo_timer(ui, pomo_go);
        });
}

fn pomo_timer(ui: &mut Ui, pomo_go: &mut PomoGo) {
    let (m, s) = pomo_go.timer.timer(pomo_go.is_running);

    let time = format!("{:02}:{:02}", m, s);

    ui.centered_and_justified(|ui| {
        ui.label(egui::RichText::new(time).size(200.).color(Color32::WHITE))
    });
}

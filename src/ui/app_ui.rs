use eframe::wgpu::Color;
use egui::{Align, Button, Color32, CornerRadius, Layout, RichText, Ui};
use egui_alignments::center_vertical;

use crate::{PomoGo, PomoTimerState, core::background::set_bg::set_bg};

pub fn pomo_ui(pomo_go: &mut PomoGo, ui: &mut Ui) {
    let bg = set_bg(&pomo_go.pomo_state);
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(bg))
        .show(ui, |ui| {
            center_vertical(ui, |ui| {
                pomo_timer(ui, pomo_go);
                toggle_timer_button(pomo_go, ui);
            });
        });
}

fn pomo_timer(ui: &mut Ui, pomo_go: &mut PomoGo) {
    let (m, s) = pomo_go.timer.timer(&pomo_go.is_running);

    if matches!(m, 0) && matches!(s, 0) && matches!(pomo_go.is_running, PomoTimerState::Running) {
        pomo_go.toggle_pomo_state();
    }

    let time = format!("{:02}:{:02}", m, s);

    ui.add(
        egui::Label::new(
            egui::RichText::new(time)
                .color(Color32::WHITE)
                .font(egui::FontId {
                    size: 200.,
                    family: egui::FontFamily::Name("Inter-Bold".into()),
                }),
        )
        .wrap_mode(egui::TextWrapMode::Extend),
    );
}

fn toggle_timer_button(pomo_go: &mut PomoGo, ui: &mut Ui) {
    let button_text = match pomo_go.is_running {
        PomoTimerState::Paused => "start",
        PomoTimerState::Running => "stop",
        PomoTimerState::Finished => unreachable!(),
    };
    let toggle_button = Button::new(RichText::new(button_text).color(Color32::BLACK).font(
        egui::FontId {
            size: 20.,
            family: egui::FontFamily::Name("Inter-Bold".into()),
        },
    ))
    .corner_radius(CornerRadius::same(255))
    .fill(Color32::WHITE);

    ui.spacing_mut().button_padding = egui::vec2(70., 15.);
    let button = ui.add(toggle_button);

    if button.clicked() {
        pomo_go.toggle_timer_state()
    }
}

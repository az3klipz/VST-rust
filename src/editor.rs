use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::Arc;
use crate::AntigravityParams;

// Import our custom UI module
use crate::ui::knob::Knob;
use crate::ui::spectrum::SpectrumAnalyzer;

pub fn create_antigravity_editor(params: Arc<AntigravityParams>) -> Option<Box<dyn Editor>> {
    let egui_state = EguiState::from_size(800, 600);
    // Move spectrum analyzer state capture into the editor closure if possible,
    // or just instantiate it per frame for this stateless prototype (less efficient but works for now)
    
    create_egui_editor(
        egui_state,
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            // Antigravity Theme
            let mut visuals = egui::Visuals::dark();
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(15, 15, 20);
            visuals.window_fill = egui::Color32::from_rgb(15, 15, 20);
            visuals.selection.bg_fill = egui::Color32::from_rgb(0, 255, 200).linear_multiply(0.2);
            visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 200));
            egui_ctx.set_visuals(visuals);

            egui::CentralPanel::default().show(egui_ctx, |ui| {
                // Header
                ui.vertical_centered(|ui| {
                     ui.add_space(20.0);
                     ui.label(egui::RichText::new("ANTIGRAVITY DESIGNER").size(24.0).strong().color(egui::Color32::from_rgb(200, 200, 220)));
                     ui.label(egui::RichText::new("Hybrid AI Framework").size(12.0).weak());
                     ui.add_space(20.0);
                     ui.separator();
                });

                ui.add_space(30.0);

                // Main Controls (Knobs)
                ui.horizontal_centered(|ui| {
                    let knob_size = 80.0; // Big "Designer" knobs

                    ui.vertical(|ui| {
                        ui.label("DRIVE");
                        ui.add(Knob::new(&params.drive).size(knob_size));
                    });

                    ui.add_space(50.0);

                    ui.vertical(|ui| {
                        ui.label("OUTPUT");
                        ui.add(Knob::new(&params.output_gain).size(knob_size));
                    });
                });

                ui.add_space(40.0);

                // Spectrum Analyzer Section
                ui.vertical_centered(|ui| {
                    ui.label("REAL-TIME ANALYSIS");
                    ui.add_space(5.0);
                    
                    // Box the analyzer
                    egui::Frame::canvas(ui.style()).fill(egui::Color32::from_rgb(10, 10, 12)).show(ui, |ui| {
                        let mut spectrum = SpectrumAnalyzer::new();
                        spectrum.ui(ui);
                    });
                });

                // Footer
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                         ui.label("Cloud Sync: ");
                         ui.colored_label(egui::Color32::RED, "OFFLINE");
                    });
                });
            });
        },
    )
}

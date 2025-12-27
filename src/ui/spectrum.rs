use nih_plug_egui::egui::{self, Ui};
use nih_plug_egui::egui_plot::{Plot, PlotPoints, Line};

pub struct SpectrumAnalyzer {
    // In a real implementation, this would read from an atomic triple buffer written by the audio thread.
    // For this prototype, we will display a simulated dynamic curve.
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let height = 100.0;
        
        Plot::new("spectrum")
            .height(height)
            .show_background(false)
            .show_axes([false, false])
            .show_x(false)
            .show_y(false)
            .allow_drag(false)
            .allow_zoom(false)
            .show(ui, |plot_ui| {
                // Mock Data: Sine wave + Noise
                let time = ui.input(|i| i.time);
                let points: PlotPoints = (0..100).map(|i| {
                    let x = i as f64;
                    let y = (x * 0.1 + time).sin() * 0.5 + 0.5; // Moving wave
                    [x, y]
                }).collect();

                plot_ui.line(Line::new(points).color(egui::Color32::from_rgb(0, 255, 200)).width(2.0));
            });
    }
}

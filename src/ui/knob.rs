use nih_plug::prelude::Param;
use nih_plug_egui::egui::{self, Response, Ui, Widget, Sense, Vec2, Pos2, Color32, Stroke};
use std::f32::consts::TAU;

pub struct Knob<'a, P: Param> {
    param: &'a P,
    size: f32,
}

impl<'a, P: Param> Knob<'a, P> {
    pub fn new(param: &'a P) -> Self {
        Self { param, size: 40.0 }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a, P: Param> Widget for Knob<'a, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::drag());

        let value = self.param.normalized_value(); // 0.0 to 1.0

        if response.dragged() {
            let delta = response.drag_delta().y * -0.005; // Drag up to increase
            let new_norm = (value + delta).clamp(0.0, 1.0);
            self.param.set_normalized_value(new_norm);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = self.size * 0.45;
            let painter = ui.painter();

            // Background
            painter.circle_filled(center, radius, Color32::from_rgb(30, 30, 35));
            painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_rgb(60, 60, 70)));

            // Arc
            // 0 is down (standard knob) -> start -5pi/4, end pi/4?
            // Egui angle: 0 is Right, CW.
            // Let's ensure a standard 270 degree arc.
            let start_angle = TAU * 0.75; // Top
            // Actually, let's just do a simple arc from 135 deg to 405 deg (270 deg span)
            // 135 deg = 2.356 rad
            // 405 deg = 7.068 rad
            let angle_min = TAU * 0.35;
            let angle_max = TAU * 1.15;
            let angle = egui::lerp(angle_min..=angle_max, value);

            // Draw "active" arc (this is complex in raw egui, simplifying to a pointer for now)
            let pointer_len = radius * 0.8;
            let pointer_pos = center + Vec2::from_angle(angle) * pointer_len;
            
            painter.line_segment([center, pointer_pos], Stroke::new(2.5, Color32::from_rgb(0, 255, 200))); // Cyan glow

            // Dynamic Glow (Antigravity Aesthetic)
            if response.hovered() || response.dragged() {
                 painter.circle_stroke(center, radius * 1.1, Stroke::new(2.0, Color32::from_rgb(0, 255, 200).linear_multiply(0.3)));
            }
        }

        response
    }
}
